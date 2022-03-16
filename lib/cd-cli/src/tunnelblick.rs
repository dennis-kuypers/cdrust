use crate::prelude::*;

pub const TUNNELBLICK_CONFIG: &str = "tunnelblick";

#[derive(Deserialize)]
pub struct TunnelblickConfig {
    pub connection: Option<String>,
}

#[derive(new)]
pub struct Tunnelblick {
    name: String,
}

pub trait TunnelblickProvider {
    fn tunnelblick(&self) -> anyhow::Result<Tunnelblick>;
}

impl TunnelblickProvider for App {
    fn tunnelblick(&self) -> anyhow::Result<Tunnelblick> {
        let config: crate::tunnelblick::TunnelblickConfig = self.config(crate::tunnelblick::TUNNELBLICK_CONFIG)?;
        let connection = config
            .connection
            .ok_or_else(|| anyhow!("No connection name provided"))?;

        Ok(Tunnelblick::new(connection))
    }
}

impl Tunnelblick {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_connected(&self) -> anyhow::Result<bool> {
        let status = cd_tunnelblick::get_status()?;

        Ok(status
            .iter()
            .any(|s| s.name == self.name && s.state == cd_tunnelblick::State::Connected))
    }
}

// FIXME: Can not be interrupted by SIGINT/ctrlc
pub fn wait_for_state<F>(wait: std::time::Duration, retries: u32, f: F) -> anyhow::Result<bool>
where
    F: Fn(Vec<cd_tunnelblick::Vpn>) -> anyhow::Result<bool>,
{
    for _ in 1..=retries {
        let status = cd_tunnelblick::get_status()?;
        match f(status) {
            Ok(false) => std::thread::sleep(wait),
            failure_or_success => return failure_or_success,
        }
    }

    Ok(false)
}
