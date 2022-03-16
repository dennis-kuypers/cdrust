//! # VPN
//!
//! Allows controlling VPN connections.
//!
use crate::prelude::*;
use crate::tunnelblick::TunnelblickProvider;

pub const VPN_CONFIG: &str = "vpn";

#[derive(Deserialize)]
pub struct VpnConfig {
    #[serde(default)]
    pub action: VpnAction,
}

#[derive(Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum VpnAction {
    Abort,
    None,
}

impl Default for VpnAction {
    fn default() -> Self {
        Self::Abort
    }
}

pub trait EnsureVpn {
    fn ensure_vpn(&self) -> anyhow::Result<()>;
}

impl EnsureVpn for App {
    fn ensure_vpn(&self) -> anyhow::Result<()> {
        let vpn_config: VpnConfig = self.config(VPN_CONFIG)?;

        // check for `none` first, because other options require tunnelblick
        if vpn_config.action == VpnAction::None {
            return Ok(());
        }

        let tunnelblick = self.tunnelblick()?;
        match vpn_config.action {
            VpnAction::Abort => {
                if !tunnelblick.is_connected()? {
                    bail!("VPN connection is not active. Aborting.");
                }
            }
            VpnAction::None => unreachable!(),
        }

        Ok(())
    }
}
