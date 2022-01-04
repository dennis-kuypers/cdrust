pub mod dialog;
pub mod ec2;

use crate::prelude::*;

const AWS_CONFIG_KEY: &str = "aws";

#[derive(Deserialize)]
pub struct AwsConfig {
    /// The profile to use
    pub profile: String,

    /// The way to handle vpn connections
    pub vpn: Option<AutoVpnMode>,
}

#[derive(Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AutoVpnMode {
    Ask,
    Abort,
}

#[async_trait::async_trait]
pub trait AwsProvider {
    async fn aws_client(&self) -> anyhow::Result<cd_aws::AwsClient>;
}

#[async_trait::async_trait]
impl AwsProvider for App {
    async fn aws_client(&self) -> anyhow::Result<cd_aws::AwsClient> {
        let aws_config: AwsConfig = self.config(AWS_CONFIG_KEY)?;

        // // check for VPN support
        // if let Some(mode) = aws_config.vpn {
        //     let vpn_config: VpnConfig = self.config.focus("vpn").extract()?;
        //
        //     ensure!(vpn_config.name.is_some(), "Can not ensure that VPN is connected (as requested by `aws.vpn`) because `vpn.name` is not configured for the current profile");
        //     let name = vpn_config.name.unwrap();
        //
        //     if mode == AutoVpnMode::Abort {
        //         let statuses = ce_tunnelblick::get_status()?;
        //         let status = statuses
        //             .iter()
        //             .find(|s| s.name == name)
        //             .ok_or_else(|| anyhow!("No vpn config with name bla active"))?;
        //         if status.state != State::Connected {
        //             bail!("VPN is not connected. Please connect first.");
        //         }
        //     }
        // }

        let client = cd_aws::AwsClient::new(&aws_config.profile).await?;

        Ok(client)
    }
}
