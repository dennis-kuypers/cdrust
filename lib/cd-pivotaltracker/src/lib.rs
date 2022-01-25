mod models;

pub use models::*;
use reqwest::header::HeaderValue;
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde::Serialize;
use thiserror::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Unknown error occurred")]
    Unknown,
    #[error(transparent)]
    ConnectionError(#[from] reqwest::Error),
}

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new(token: &str) -> Self {
        use reqwest::{
            header::{HeaderMap, HeaderName},
            ClientBuilder,
        };

        let token = HeaderValue::from_str(token).expect("Failed to parse token");

        let mut headers = HeaderMap::new();
        headers.insert(HeaderName::from_static("x-trackertoken"), token);

        let client = ClientBuilder::new()
            .default_headers(headers)
            .build()
            .expect("Failed to initialize HTTP library");

        Self { client }
    }

    async fn get<T: DeserializeOwned>(&self, url: impl reqwest::IntoUrl) -> Result<T> {
        Ok(self.client.get(url).send().await?.error_for_status()?.json().await?)
    }

    async fn put<D: Serialize, R: DeserializeOwned>(&self, url: impl reqwest::IntoUrl, data: D) -> Result<R> {
        Ok(self
            .client
            .put(url)
            .json(&data)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }

    async fn post<D: Serialize, R: DeserializeOwned>(&self, url: impl reqwest::IntoUrl, data: D) -> Result<R> {
        Ok(self
            .client
            .post(url)
            .json(&data)
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?)
    }
    pub async fn get_story(&self, project_id: u64, story_id: u64) -> Result<Story> {
        let url = format!(
            "https://www.pivotaltracker.com/services/v5/projects/{project_id}/stories/{story_id}?fields=project_id,name,description,requested_by,url,story_type,estimate,current_state,created_at,updated_at,accepted_at,owners,labels,tasks,pull_requests,comments,transitions",
        );
        self.get(url).await
    }

    pub async fn get_stories(&self, project_id: u64, filter: &str) -> Result<Vec<Story>> {
        let url = Url::parse_with_params(
            &format!("https://www.pivotaltracker.com/services/v5/projects/{project_id}/stories/?fields=project_id,name,description,requested_by,url,story_type,estimate,current_state,created_at,updated_at,accepted_at,owners,labels,tasks,pull_requests,comments,transitions",),
            &[("filter", filter)],
        )
        .unwrap();

        self.get(url).await
    }

    pub async fn get_project_members(&self, project_id: u64) -> Result<Vec<ProjectMember>> {
        let url = format!("https://www.pivotaltracker.com/services/v5/projects/{project_id}/memberships?fields=person");
        self.get(url).await
    }

    pub async fn start_story(&self, project_id: u64, story_id: u64) -> Result<Story> {
        let url = format!("https://www.pivotaltracker.com/services/v5/projects/{project_id}/stories/{story_id}");

        #[derive(Debug, Serialize)]
        struct StoryRequest {
            current_state: StoryState,
        }
        let data = StoryRequest {
            current_state: StoryState::Started,
        };

        self.put(url, data).await
    }

    pub async fn create_task(
        &self,
        project_id: u64,
        story_id: u64,
        position: usize,
        description: &str,
    ) -> Result<Story> {
        let url = format!("https://www.pivotaltracker.com/services/v5/projects/{project_id}/stories/{story_id}/tasks");

        #[derive(Serialize)]
        struct CreateTaskRequest<'a> {
            pub description: &'a str,
            pub position: usize,
        }
        let data = CreateTaskRequest {
            description: &format!("{}. {}", position, description),
            position,
        };
        self.post(url, data).await
    }

    pub async fn set_description(&self, project_id: u64, story_id: u64, description: &str) -> Result<Story> {
        let url = format!("https://www.pivotaltracker.com/services/v5/projects/{project_id}/stories/{story_id}");

        #[derive(Serialize)]
        struct SetDescription<'a> {
            pub description: &'a str,
        }
        let data = SetDescription { description };

        self.put(url, data).await
    }
}
