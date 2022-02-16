use cd_cli::dialog::DialogProvider;
use cd_cli::pivotal::{PivotalTracker, SelectStory};
use cd_cli::prelude::*;
use cd_pivotaltracker::{Story, StoryState};
use git2::build::CheckoutBuilder;
use git2::{BranchType, ErrorCode, FetchOptions, RemoteCallbacks};

#[derive(StructOpt, Debug)]
#[structopt(name = env!("CARGO_BIN_NAME"), version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Opts {
    #[structopt(flatten)]
    pub config: cd_cli::config::ConfigOpts,

    #[structopt(flatten)]
    pub dialog: cd_cli::dialog::DialogOpts,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(alias = "prep")]
    /// Prepares a story with the default description and tasks (alias: prep)
    Prepare(SelectStory),

    /// Switches to a different story
    On(SelectStory),

    Test,
}

// TODO: Make configurable
static GIT_REMOTE_NAME: &str = "origin";

static RISK_ASSESSMENT_MARKER: &str = "# Risk Assessment";
static RISK_ASSESSMENT_DEFAULT: &str = r#"# Risk Assessment

There is no specific risk according to the common risk criteria defined in the DevOps handbook, chapter "Development Process"."#;

static TASKS: &[&str] = &[
    "Detail Planning including Risk Assessment",
    "Document Risk Assessment",
    "Create Feature Branch",
    "Development including Tests and Documentation",
    "Create Pull Request and Request Review",
    "Request Functional Acceptance",
    "Prepare Deployment",
    "Request Deployment/Merge Approval",
    "Merge Pull Request",
    "Execute Deployment",
    "Verify Production Delivery",
];

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();
    let app = cd_cli::app::App::new(env!("CARGO_BIN_NAME"), opts.config.clone())?;

    use cd_cli::pivotal::PivotalTrackerProvider;
    let pivotal_client = app.pivotal_tracker_client()?;

    match opts.cmd {
        Command::Prepare(cmd_opts) => prepare(cmd_opts, pivotal_client, app.dialog(opts.dialog)?).await,
        Command::On(cmd_opts) => on(cmd_opts, pivotal_client, app.dialog(opts.dialog)?).await,
        Command::Test => {
            let repo_path = std::env::var("INFRA").unwrap();
            let repo = git2::Repository::open(repo_path).unwrap();
            let result = repo.find_branch(
                &format!("{}/PT-178352361-tflint-unused-declarations", GIT_REMOTE_NAME),
                BranchType::Remote,
            );
            match result {
                Ok(b) => eprintln!("N:{:?}, UPSTREAM:{:?}", b.name(), b.upstream().unwrap().name().unwrap()),
                Err(e) => eprintln!("{e:?}"),
            }
            Ok(())
        }
    }
}

fn abc(url: &str, username: Option<&str>, allowed: git2::CredentialType) -> Result<git2::Cred, git2::Error> {
    let mut cred_helper = git2::CredentialHelper::new(url);
    let cfg = git2::Config::open_default().unwrap();
    cred_helper.config(&cfg);

    if allowed.contains(git2::CredentialType::SSH_KEY) {
        let user = username
            .map(|s| s.to_string())
            .or_else(|| cred_helper.username.clone())
            .unwrap_or_else(|| "git".to_string());
        git2::Cred::ssh_key_from_agent(&user)
    } else if allowed.contains(git2::CredentialType::USER_PASS_PLAINTEXT) {
        git2::Cred::credential_helper(&cfg, url, username)
    } else if allowed.contains(git2::CredentialType::DEFAULT) {
        git2::Cred::default()
    } else {
        Err(git2::Error::from_str("no authentication available"))
    }
}

async fn on(
    opts: SelectStory,
    client: PivotalTracker,
    dialog: cd_cli::dialog::InteractiveOutput,
) -> anyhow::Result<()> {
    let story = client.user_select_story(opts, &dialog).await?;
    prepare_tasks(&client, &story).await?;
    prepare_risk_assessment(&client, &story).await?;

    trace!("Story estimate: {:?}", story.estimate);
    let story_unestimated = story.estimate.map(|e| e == -1f32).unwrap_or(true);
    if story_unestimated {
        debug!("story is not estimated");

        let estimate = dialog
            .read_line(
                "The story is not estimated. Please provide an estimate (0,1,2,3)",
                Some("0".to_string()),
            )
            .ok_or_else(|| anyhow!("User provided no estimate. Aborting"))?;
        let estimate = match estimate.as_str() {
            "0" => 0f32,
            "1" => 1f32,
            "2" => 2f32,
            "3" => 3f32,
            e => bail!("{e} is not a valid estimate"),
        };
        client
            .set_story_estimate(story.id, estimate)
            .await
            .with_context(|| "Failed to set story estimate")?;
    }

    // check the state
    match story.current_state {
        Some(StoryState::Started) => {} // already started, no change in state required
        Some(StoryState::Unstarted) => {
            client.set_story_state(story.id, StoryState::Started).await?;
        }
        Some(StoryState::Planned) => {
            client.set_story_state(story.id, StoryState::Started).await?;
        }
        Some(StoryState::Rejected) => {
            client.set_story_state(story.id, StoryState::Started).await?;
        }
        _ => bail!("Story should be in a workable state (started, unstarted, planned, rejected) to work on it"),
    };

    // TODO: Make ENV or PATH to repo configurable
    let repo_path = std::env::var("INFRA").unwrap();
    let repo = git2::Repository::open(repo_path).unwrap();

    // Evaluate current branches and PRs
    if !story.branches.is_empty() {
        debug!("Found {} existing branches in story", story.branches.len());
        // let user select an existing branch or choose to select a new one
        let mut choices = vec!["Create a new branch"];
        choices.extend(story.branches.iter().map(|b| b.name.as_str()));
        let choice = dialog
            .select_one(&choices, "Found existing branches. Which do you want to work on?")
            .ok_or_else(|| anyhow!("No branch selected"))?;
        if choice > 0 {
            let branch = &story.branches[choice - 1];
            debug!("User selected existing branch {}", branch.name);

            let local_branch = repo.find_branch(&branch.name, BranchType::Local);
            match local_branch {
                Err(e) if e.code() == ErrorCode::NotFound => {
                    bail!("The branch does not exist. This is currently not implemented");
                }
                Ok(b) => {
                    debug!("Fetching git branch");
                    let mut cb = RemoteCallbacks::new();
                    cb.credentials(abc);
                    repo.find_remote(GIT_REMOTE_NAME)
                        .expect("Could not find remote repo in your git repository")
                        .fetch(&[&branch.name], Some(FetchOptions::new().remote_callbacks(cb)), None)?;
                    let remote_branch =
                        repo.find_branch(&format!("{}/{}", GIT_REMOTE_NAME, branch.name), BranchType::Remote)?;
                    repo.set_head(b.get().name().unwrap())?;
                    let (analysis, _) = repo.merge_analysis(&[&repo.reference_to_annotated_commit(b.get())?])?;
                    if !analysis.is_up_to_date() {
                        ensure!(
                            analysis.is_fast_forward(),
                            "The remote changes can not be merged by a fast forward. Manual intervention required."
                        );

                        b.into_reference().set_target(
                            remote_branch
                                .into_reference()
                                .target()
                                .expect("Expected branch to exist"),
                            "cwk fast forwarded remote branch",
                        )?;
                    }
                    repo.checkout_head(Some(CheckoutBuilder::new().safe())).context(
                        "Unable to checkout desired branch safely. Make sure that index and workspace are clean.",
                    )?;
                }

                Err(e) => {
                    bail!("Git said {:?}. This can currently not be handled correctly, sorry.", e);
                }
            }

            return Ok(());
        }
        // fall through to creating a new branch
    }

    let suffix_suggestion = mangle_story_name(&story.name.unwrap());

    let mut branch_prefix = format!("PT-{}-", story.id);
    let branch_suffix = dialog.read_line("Branch suffix?", Some(suffix_suggestion)).unwrap();
    branch_prefix.push_str(&branch_suffix);
    let new_branch = repo.branch(&branch_prefix, &repo.head()?.peel_to_commit()?, true)?;
    repo.set_head(new_branch.get().name().unwrap())?;

    Ok(())
}

/// use only lowercase alphanumeric characters and replace others with a dash `-`
fn mangle_story_name(story_name: &str) -> String {
    let name = story_name.chars().fold(String::new(), |mut s, c| {
        match c {
            'a'..='z' => s.push(c),
            'A'..='Z' => s.push(c.to_ascii_lowercase()),
            '0'..='9' => s.push(c),
            _ => {
                if !s.ends_with('-') {
                    s.push('-')
                }
            }
        };
        s
    });
    name.trim_end_matches('-').to_string()
}

#[instrument(skip_all)]
async fn prepare_tasks(client: &PivotalTracker, story: &Story) -> anyhow::Result<()> {
    // Get a sorted list in preparation of some more intelligent comparisons
    // not really necessary at the moment.
    let mut tasks = Vec::new();
    tasks.clone_from(&story.tasks);
    tasks.sort_by_key(|t| t.position);

    let existing_count = tasks.len();
    let configured_count = TASKS.len();

    // This block just performs user feedback - actual logic below
    match (existing_count, configured_count) {
        (_, 0) => warn!("No tasks configured. Nothing to do."),
        (0, c) => debug!("Creating {} tasks", c),
        (e, c) if e > c => warn!("There are already {} tasks in the story but your configuration only specifies {} tasks. Not doing anything.", e, c),
        (e, c) if e == c => debug!("Story already has {} tasks. Skipping", e),
        (e, c) => debug!("Creating remaining {} of {} tasks", c-e, c),
    }

    // This loop does nothing when `expected <= actual` thanks to the `skip()`
    for (id, text) in TASKS
        .iter()
        .enumerate()
        .map(|(i, t)| (i + 1, t)) // we count starting from 1
        .skip(existing_count)
    {
        debug!("Creating task {}/{}", id, configured_count);
        client.create_story_task(story.id, id, text).await?;
    }

    Ok(())
}

async fn prepare_risk_assessment(client: &PivotalTracker, story: &Story) -> anyhow::Result<()> {
    if story
        .description
        .as_ref()
        .map(|d| !d.contains(RISK_ASSESSMENT_MARKER))
        .unwrap_or(true)
    {
        debug!("Updating story description");

        let mut new_description = story
            .description
            .clone()
            .map(|mut e| {
                // append newlines to existing description
                e.push_str("\n\n");
                e
            })
            .unwrap_or_default();

        new_description.push_str(RISK_ASSESSMENT_DEFAULT);

        client.set_story_description(story.id, &new_description).await?;
    } else {
        debug!("Story text contains desired snippet. Skipping.");
    }

    Ok(())
}

async fn prepare(
    opts: SelectStory,
    client: PivotalTracker,
    dialog: cd_cli::dialog::InteractiveOutput,
) -> anyhow::Result<()> {
    let story = client.user_select_story(opts, &dialog).await?;

    prepare_tasks(&client, &story).await?;
    prepare_risk_assessment(&client, &story).await?;

    Ok(())
}
