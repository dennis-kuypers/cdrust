pub mod ec2;

use aws_config::meta::region::ProvideRegion;
use aws_config::provider_config::ProviderConfig;
use aws_sdk_ec2::SdkError;
use thiserror::Error;

pub struct AwsClient(pub(crate) aws_types::config::Config);

#[derive(Error, Debug)]
pub enum AwsClientError {
    #[error("The aws configuration is not valid")]
    InvalidConfig,
    #[error("Request failed")]
    SdkError(Box<dyn std::error::Error + Send + Sync>),
}

impl<E, R> From<aws_sdk_ec2::SdkError<E, R>> for AwsClientError
where
    E: 'static + std::error::Error + Send + Sync,
    R: 'static + Send + Sync + std::fmt::Debug,
{
    fn from(e: SdkError<E, R>) -> Self {
        AwsClientError::SdkError(Box::new(e))
    }
}

impl AwsClient {
    /// Loads AWS rust SDK config object
    /// We are using the files in ~/.aws/* to get credentials
    pub async fn new(profile: &str) -> Result<AwsClient, ConfigLoadError> {
        use aws_config::{
            profile::{credentials::ProfileFileCredentialsProvider, ProfileFileRegionProvider},
            ConfigLoader,
        };

        if profile.is_empty() {
            return Err(ConfigLoadError::NoProfileGiven);
        }

        let provider_config = ProviderConfig::without_region();
        let region_provider = ProfileFileRegionProvider::builder()
            .configure(&provider_config)
            .profile_name(profile)
            .build();

        let region = region_provider.region().await;

        let credentials_provider = ProfileFileCredentialsProvider::builder()
            .configure(&provider_config.with_region(region.clone()))
            .profile_name(profile)
            .build();

        let aws_config = ConfigLoader::default()
            .region(region.unwrap())
            .credentials_provider(credentials_provider)
            .load()
            .await;

        Ok(AwsClient(aws_config))
    }
}

#[derive(Error, Debug)]
pub enum ConfigLoadError {
    #[error("No profile specified")]
    NoProfileGiven,
}
