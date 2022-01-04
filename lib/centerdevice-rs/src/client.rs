pub mod auth;
pub mod collections;
pub mod delete;
pub mod download;
pub mod search;
pub mod upload;
pub mod users;

pub use auth::{Code, CodeProvider, Token};

use crate::{
    client::{
        collections::{CollectionsQuery, CollectionsResult},
        download::Download,
        search::{Search, SearchResult},
        upload::Upload,
        users::{UsersQuery, UsersResult},
    },
    errors::{Error, ErrorKind, Result},
    CenterDevice, ClientCredentials, WithProgress,
};

use failure::Fail;
use reqwest::{self, blocking::Response, IntoUrl, StatusCode};

pub struct UnauthorizedClient<'a> {
    pub(crate) base_url: &'a str,
    pub(crate) client_credentials: ClientCredentials<'a>,
    pub(crate) http_client: reqwest::blocking::Client,
}

impl<'a, 'b: 'a> UnauthorizedClient<'b> {
    pub fn authorize_with_code_flow<T: IntoUrl + ToString + Clone, S: CodeProvider>(
        self,
        redirect_uri: T,
        code_provider: &S,
    ) -> Result<AuthorizedClient<'a>> {
        let redirect_url = redirect_uri
            .clone()
            .into_url()
            .map_err(|e| e.context(ErrorKind::FailedToPrepareHttpRequest(redirect_uri.to_string())))?;

        let token = auth::authorization_code_flow(
            &self.client_credentials,
            self.base_url,
            &redirect_url,
            code_provider,
            &self.http_client,
        )?;

        let authorized_client = AuthorizedClient {
            base_url: self.base_url,
            client_credentials: self.client_credentials,
            token,
            http_client: self.http_client,
        };

        Ok(authorized_client)
    }
}

pub struct AuthorizedClient<'a> {
    pub(crate) base_url: &'a str,
    pub(crate) client_credentials: ClientCredentials<'a>,
    pub(crate) token: Token,
    pub(crate) http_client: reqwest::blocking::Client,
}

impl<'a> AuthorizedClient<'a> {
    pub fn token(&self) -> &Token {
        &self.token
    }
}

impl<'a> CenterDevice for AuthorizedClient<'a> {
    fn refresh_access_token(&self) -> Result<Token> {
        auth::refresh_access_token(self)
    }

    fn search_documents(&self, search: Search) -> Result<SearchResult> {
        search::search_documents(self, search)
    }

    fn upload_file(&self, upload: Upload) -> Result<ID> {
        upload::upload_file(&self, upload)
    }

    fn download_file(&self, download: Download) -> Result<u64> {
        download::download_file(self, download)
    }

    fn download_file_with_progress<T: WithProgress>(&self, download: Download, progress: &mut T) -> Result<u64> {
        download::download_file_with_progress(self, download, progress)
    }

    fn delete_documents(&self, document_ids: &[&str]) -> Result<()> {
        delete::delete_documents(self, document_ids)
    }

    fn search_users(&self, users_query: UsersQuery) -> Result<UsersResult> {
        users::search_users(self, users_query)
    }

    fn search_collections(&self, collections_query: CollectionsQuery) -> Result<CollectionsResult> {
        collections::search_collections(self, collections_query)
    }
}

pub type ID = String;

pub(crate) trait GeneralErrHandler {
    type T: std::marker::Sized;

    fn general_err_handler(self, expected_states: &[StatusCode]) -> Result<Self::T>;
}

impl GeneralErrHandler for Response {
    type T = Response;

    fn general_err_handler(self, expected_states: &[StatusCode]) -> Result<Self> {
        match self.status() {
            code if expected_states.contains(&code) => Ok(self),
            code @ StatusCode::UNAUTHORIZED => Err(Error::from(ErrorKind::ApiCallFailedInvalidToken(code))),
            code @ StatusCode::TOO_MANY_REQUESTS => Err(Error::from(ErrorKind::ApiCallFailedTooManyRequests(code))),
            _ => Err(handle_error(self)),
        }
    }
}

fn handle_error(response: Response) -> Error {
    let status_code = response.status();

    match response.text() {
        Ok(body) => Error::from(ErrorKind::ApiCallFailed(status_code, body)),
        Err(e) => e
            .context(ErrorKind::FailedToProcessHttpResponse(
                status_code,
                "reading body".to_string(),
            ))
            .into(),
    }
}
