use cd_cli::prelude::*;

#[derive(StructOpt, Debug)]
#[structopt(name = env!("CARGO_BIN_NAME"), version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Opts {
    #[structopt(flatten)]
    pub config: cd_cli::config::ConfigOpts,

    // currently unused
    // #[structopt(flatten)]
    // pub dialog: cd_cli::dialog::DialogOpts,
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(alias = "prep")]
    /// Prepares a story with the default description and tasks (alias: prep)
    Prepare(SelectStory),
}

#[derive(StructOpt, Debug)]
pub struct SelectStory {
    #[structopt(parse(try_from_str = cd_cli::pivotal::parse_pivotal_story_id))]
    pub story_id: u64,
}

static PIVOTALTRACKER_CONFIG_KEY: &str = "pivotal";

#[derive(Deserialize)]
pub struct PivotalConfig {
    pub token: String,
    pub me: String,
    pub project_id: u64,
    pub story_template: StoryTemplate,
}

#[derive(Deserialize)]
pub struct StoryTemplate {
    pub description: String,
    pub tasks: Vec<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();
    let app = cd_cli::app::App::new(env!("CARGO_BIN_NAME"), opts.config.clone())?;
    let pivotal_config: PivotalConfig = app.config(PIVOTALTRACKER_CONFIG_KEY)?;

    match opts.cmd {
        Command::Prepare(args) => prepare(args, pivotal_config).await,
    }
}

async fn prepare(args: SelectStory, config: PivotalConfig) -> anyhow::Result<()> {
    let client = cd_pivotaltracker::Client::new(&config.token);
    let project_id = config.project_id;
    let story_id = args.story_id;

    let story = client.get_story(project_id, story_id).await?;

    // Get a sorted list in preparation of some more intelligent comparisons
    // not really necessary at the moment.
    let mut tasks = Vec::new();
    tasks.clone_from(&story.tasks);
    tasks.sort_by_key(|t| t.position);

    let existing_count = tasks.len();
    let configured_count = config.story_template.tasks.len();

    // This block just performs user feedback - actual logic below
    match (existing_count, configured_count) {
        (_, 0) => warn!("No tasks configured. Nothing to do."),
        (0, c) => debug!("Creating {} tasks", c),
        (e, c) if e > c => warn!("There are already {} tasks in the story but your configuration only specifies {} tasks. Not doing anything.", e, c),
        (e, c) if e == c => debug!("Story already has {} tasks. Skipping", e),
        (e, c) => debug!("Creating remaining {} of {} tasks", c-e, c),
    }

    // This loop does nothing when `expected <= actual` thanks to the `skip()`
    for (id, text) in config
        .story_template
        .tasks
        .iter()
        .enumerate()
        .map(|(i, t)| (i + 1, t)) // we count starting from 1
        .skip(existing_count)
    {
        debug!("Creating task {}/{}", id, configured_count);
        client.create_task(project_id, story_id, id, text).await?;
    }

    let requested_text = config.story_template.description;
    if story
        .description
        .as_ref()
        .map(|d| !d.contains(&requested_text))
        .unwrap_or(false)
    {
        info!("Updating story description");

        let mut new_description = story
            .description
            .map(|mut e| {
                // append newlines to existing description
                e.push_str("\n\n");
                e
            })
            .unwrap_or_default();

        new_description.push_str(&requested_text);

        client.set_description(project_id, story_id, &new_description).await?;
    } else {
        debug!("Story text contains desired snippet. Skipping.");
    }

    Ok(())
}
