pub mod client;
pub mod errors;
pub mod utils;

pub use crate::client::auth::Token;
pub use reqwest::{blocking::Client as HttpClient, Certificate};

use crate::{
    client::{
        collections::{CollectionsQuery, CollectionsResult},
        download::Download,
        search::{Search, SearchResult},
        upload::Upload,
        users::{UsersQuery, UsersResult},
        AuthorizedClient, UnauthorizedClient, ID,
    },
    errors::Result,
};

static APP_USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"),);

pub trait CenterDevice {
    fn refresh_access_token(&self) -> Result<Token>;
    fn search_documents(&self, search: Search) -> Result<SearchResult>;
    fn upload_file(&self, upload: Upload) -> Result<ID>;
    fn download_file(&self, download: Download) -> Result<u64>;
    fn download_file_with_progress<T: WithProgress>(&self, download: Download, progress: &mut T) -> Result<u64>;
    fn delete_documents(&self, document_ids: &[&str]) -> Result<()>;
    fn search_users(&self, users_query: UsersQuery) -> Result<UsersResult>;
    fn search_collections(&self, collections_query: CollectionsQuery) -> Result<CollectionsResult>;
}

pub struct ClientBuilder<'a> {
    base_url: &'a str,
    client_credentials: ClientCredentials<'a>,
    root_cert: Option<Certificate>,
}

impl<'a> ClientBuilder<'a> {
    pub fn new(base_url: &'a str, client_credentials: ClientCredentials<'a>) -> ClientBuilder<'a> {
        ClientBuilder {
            base_url,
            client_credentials,
            root_cert: None,
        }
    }

    pub fn add_root_certificate(self, certificate: Certificate) -> Self {
        Self {
            root_cert: Some(certificate),
            ..self
        }
    }

    pub fn build(self) -> UnauthorizedClient<'a> {
        let http_client = Self::build_http_client(self.root_cert);
        Client::new(self.base_url, self.client_credentials, http_client)
    }

    pub fn build_with_token(self, token: Token) -> AuthorizedClient<'a> {
        let http_client = Self::build_http_client(self.root_cert);
        Client::with_token(self.base_url, self.client_credentials, token, http_client)
    }

    fn build_http_client(root_cert: Option<Certificate>) -> HttpClient {
        let mut client_builder = reqwest::blocking::Client::builder().user_agent(APP_USER_AGENT);
        if let Some(cert) = root_cert {
            client_builder = client_builder.add_root_certificate(cert)
        };
        client_builder.build().expect("Failed to build HTTP client")
    }
}

pub struct Client {}

impl Client {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<'a>(
        base_url: &'a str,
        client_credentials: ClientCredentials<'a>,
        http_client: HttpClient,
    ) -> UnauthorizedClient<'a> {
        UnauthorizedClient {
            base_url,
            client_credentials,
            http_client,
        }
    }

    pub fn with_token<'a>(
        base_url: &'a str,
        client_credentials: ClientCredentials<'a>,
        token: Token,
        http_client: HttpClient,
    ) -> AuthorizedClient<'a> {
        AuthorizedClient {
            base_url,
            client_credentials,
            token,
            http_client,
        }
    }
}

#[derive(Debug)]
pub struct ClientCredentials<'a> {
    client_id: &'a str,
    client_secret: &'a str,
}

impl<'a> ClientCredentials<'a> {
    pub fn new(client_id: &'a str, client_secret: &'a str) -> ClientCredentials<'a> {
        ClientCredentials {
            client_id,
            client_secret,
        }
    }
}

pub trait WithProgress {
    fn setup(&mut self, size: usize);
    fn progress(&mut self, amount: usize);
    fn finish(&self);
}
