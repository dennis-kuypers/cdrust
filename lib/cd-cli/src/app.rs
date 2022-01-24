use crate::config::{load_config, ConfigOpts};
use crate::prelude::*;

pub struct App {
    config: figment::Figment,
}

impl App {
    pub fn new(name: &str, config: ConfigOpts) -> anyhow::Result<Self> {
        let config = load_config(config, name);

        let env_log = format!("{}_LOG", name.replace('-', "_").to_uppercase());
        tracing_subscriber::FmtSubscriber::builder()
            .with_max_level(tracing::Level::ERROR)
            .with_env_filter(tracing_subscriber::EnvFilter::from_env(env_log))
            .with_writer(std::io::stderr)
            .init();

        Ok(App { config })
    }

    pub fn config<'a, T: serde::Deserialize<'a>>(&self, name: &str) -> anyhow::Result<T> {
        self.config
            .focus(name)
            .extract()
            .map_err(|e| anyhow!("Failed to load configuration for {}. {}", name, e))
    }
}
