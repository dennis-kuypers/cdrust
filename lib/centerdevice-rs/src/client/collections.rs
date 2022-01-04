use crate::{
    client::{AuthorizedClient, GeneralErrHandler, ID},
    errors::{Error, ErrorKind, Result},
};

use chrono::{DateTime, FixedOffset};
use failure::Fail;
use log::debug;
use reqwest::{blocking::Response, StatusCode};
use serde::{self, Deserialize, Serialize};
use std::string::ToString;

#[derive(Serialize, Debug)]
pub struct CollectionsQuery<'a> {
    include_public: bool,
    name: Option<&'a str>,
    ids: Option<Vec<&'a str>>,
}

impl<'a> Default for CollectionsQuery<'a> {
    fn default() -> Self {
        CollectionsQuery {
            include_public: false,
            name: None,
            ids: None,
        }
    }
}

impl<'a> CollectionsQuery<'a> {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn include_public(self) -> CollectionsQuery<'a> {
        CollectionsQuery {
            include_public: true,
            ..self
        }
    }

    pub fn name(self, name: &'a str) -> CollectionsQuery<'a> {
        CollectionsQuery {
            name: Some(name),
            ..self
        }
    }

    pub fn ids(self, ids: Vec<&'a str>) -> CollectionsQuery<'a> {
        CollectionsQuery { ids: Some(ids), ..self }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CollectionsResult {
    pub collections: Vec<Collection>,
}

impl Default for CollectionsResult {
    fn default() -> CollectionsResult {
        CollectionsResult {
            collections: Vec::with_capacity(0),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Collection {
    pub id: String,
    pub public: bool,
    pub name: String,
    pub owner: ID,
    pub auditing: bool,
    #[serde(rename = "archived-date")]
    pub archived_date: Option<DateTime<FixedOffset>>,
    #[serde(rename = "has-folders")]
    pub has_folders: Option<bool>,
}

pub fn search_collections(
    authorized_client: &AuthorizedClient,
    collection_query: CollectionsQuery,
) -> Result<CollectionsResult> {
    let url = format!("https://api.{}/v2/collections", authorized_client.base_url);

    let ids_str; // Borrow checker
    let mut params = Vec::new();
    if collection_query.include_public {
        params.push(("include-public", "true"));
    }
    if let Some(name) = collection_query.name {
        params.push(("name", name));
    }
    if let Some(ids) = collection_query.ids {
        ids_str = ids.as_slice().join(",");
        params.push(("ids", &ids_str));
    }

    let request = authorized_client
        .http_client
        .get(&url)
        .query(&params)
        .bearer_auth(&authorized_client.token.access_token);
    debug!("Request: '{:#?}'", request);

    let response: Response = request
        .send()
        .map_err(|e| e.context(ErrorKind::HttpRequestFailed))?
        .general_err_handler(&[StatusCode::OK, StatusCode::NO_CONTENT])?;
    debug!("Response: '{:#?}'", response);

    let result = match response.status() {
        status @ StatusCode::OK => response.json().map_err(|e| {
            e.context(ErrorKind::FailedToProcessHttpResponse(
                status,
                "reading body".to_string(),
            ))
        })?,
        StatusCode::NO_CONTENT => CollectionsResult::default(),
        code => {
            return Err(Error::from(ErrorKind::ApiCallFailed(
                code,
                "unexpected response code".to_string(),
            )))
        }
    };

    Ok(result)
}
