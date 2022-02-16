use crate::dialog::InteractiveOutput;
use crate::prelude::*;
use cd_pivotaltracker::{Story, StoryState};

#[derive(StructOpt, Debug)]
pub struct SelectStory {
    #[structopt(parse(try_from_str = parse_pivotal_story_id))]
    pub story_id: Option<u64>,
}

static PIVOTALTRACKER_CONFIG_KEY: &str = "pivotal";

#[derive(Deserialize)]
pub struct PivotalConfig {
    pub token: String,
    pub me: String,
    pub project_id: u64,
}

pub trait PivotalTrackerProvider {
    fn pivotal_tracker_client(&self) -> anyhow::Result<PivotalTracker>;
}

impl PivotalTrackerProvider for App {
    fn pivotal_tracker_client(&self) -> anyhow::Result<PivotalTracker> {
        let config: PivotalConfig = self.config(PIVOTALTRACKER_CONFIG_KEY)?;
        let client = cd_pivotaltracker::Client::new(&config.token);
        Ok(PivotalTracker {
            client,
            project_id: config.project_id,
            me: config.me,
        })
    }
}

pub struct PivotalTracker {
    pub client: cd_pivotaltracker::Client,
    pub project_id: u64,
    pub me: String,
}

impl PivotalTracker {
    pub async fn set_story_description(&self, id: u64, description: &str) -> anyhow::Result<Story> {
        Ok(self.client.set_description(self.project_id, id, description).await?)
    }

    pub async fn set_story_estimate(&self, id: u64, estimate: f32) -> anyhow::Result<Story> {
        Ok(self.client.set_estimate(self.project_id, id, estimate).await?)
    }

    pub async fn set_story_state(&self, id: u64, state: StoryState) -> anyhow::Result<Story> {
        Ok(self.client.set_story_state(self.project_id, id, state).await?)
    }

    pub async fn create_story_task(
        &self,
        story_id: u64,
        task_position: usize,
        description: &str,
    ) -> anyhow::Result<Story> {
        Ok(self
            .client
            .create_task(self.project_id, story_id, task_position, description)
            .await?)
    }

    pub async fn user_select_story(&self, opts: SelectStory, dialog: &InteractiveOutput) -> anyhow::Result<Story> {
        if let Some(id) = opts.story_id {
            Ok(self.client.get_story(self.project_id, id).await?)
        } else {
            let mut stories = self
                .client
                .get_stories(
                    self.project_id,
                    &format!(
                        "owner:\"{}\" AND (state:unstarted OR state:started OR state:planned)",
                        self.me
                    ),
                )
                .await?;

            let story_text: Vec<String> = stories
                .iter()
                .map(|s| format!("[#{}] {}", s.id, s.name.as_ref().unwrap()))
                .collect();
            let story_text_ref: Vec<_> = story_text.iter().map(AsRef::as_ref).collect();
            let selected_index = dialog
                .select_one(&story_text_ref, "Select story")
                .ok_or_else(|| anyhow!("No story selected"))?;

            Ok(stories.swap_remove(selected_index))
        }
    }
}

pub fn parse_pivotal_story_id(s: &str) -> Result<u64, String> {
    // we consider everything the user may throw at us

    // 1. a u64
    if let Ok(id) = s.parse::<u64>() {
        return Ok(id);
    }

    // 2. a u64 prefixed with '#'
    if let Ok(id) = s.trim_start_matches('#').parse::<u64>() {
        return Ok(id);
    }

    // 3. a pivotal url containing a story id
    if let Ok(url) = url::Url::parse(s) {
        if let Some(mut segments) = url.path_segments() {
            // we could check the hostname but that probably doesnt help us any way
            // ensure that path is "/story/show/<ID>" though...
            if segments.next() == Some("story") && segments.next() == Some("show") {
                if let Some(id) = segments.next().and_then(|s| s.parse::<u64>().ok()) {
                    return Ok(id);
                }
            }
        }
    }
    Err("Can not parse ticket id".to_string())
}
