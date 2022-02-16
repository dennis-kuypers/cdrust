use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Story {
    pub id: u64,
    pub project_id: Option<u64>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
    pub story_type: Option<StoryType>,
    pub current_state: Option<StoryState>,
    pub estimate: Option<f32>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub accepted_at: Option<String>,
    pub requested_by: Option<Person>,
    #[serde(default)]
    pub owners: Vec<Person>,
    #[serde(default)]
    pub labels: Vec<Label>,
    #[serde(default)]
    pub tasks: Vec<Task>,
    #[serde(default)]
    pub pull_requests: Vec<PullRequest>,
    #[serde(default)]
    pub branches: Vec<Branch>,
    #[serde(default)]
    pub comments: Vec<Comment>,
    #[serde(default)]
    pub transitions: Vec<Transition>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StoryType {
    #[serde(rename = "feature")]
    Feature,
    #[serde(rename = "bug")]
    Bug,
    #[serde(rename = "chore")]
    Chore,
    #[serde(rename = "release")]
    Release,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum StoryState {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "delivered")]
    Delivered,
    #[serde(rename = "finished")]
    Finished,
    #[serde(rename = "started")]
    Started,
    #[serde(rename = "rejected")]
    Rejected,
    #[serde(rename = "planned")]
    Planned,
    #[serde(rename = "unstarted")]
    Unstarted,
    #[serde(rename = "unscheduled")]
    Unscheduled,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub initials: String,
    pub username: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Label {
    pub id: u64,
    pub name: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Task {
    pub id: u64,
    pub description: String,
    pub complete: bool,
    pub position: u64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Branch {
    pub id: u64,
    pub story_id: u64,
    pub owner: String,
    pub repo: String,
    pub name: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct PullRequest {
    pub id: u64,
    pub owner: String,
    pub repo: String,
    pub number: u64,
    pub host_url: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    pub id: u64,
    pub text: Option<String>,
    pub person_id: u64,
    pub commit_identifier: Option<String>,
    pub commit_type: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Transition {
    pub state: StoryState,
    pub occurred_at: String,
    pub performed_by_id: u64,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectMember {
    pub person: Person,
}
