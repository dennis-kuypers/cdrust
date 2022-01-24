use crate::config::{load_config, ConfigOpts};
use crate::prelude::*;

pub struct App {
    config: figment::Figment,
}

impl App {
    pub fn new(name: &str, config: ConfigOpts) -> anyhow::Result<Self> {
        let config = load_config(config, name);

        let logging_config: crate::logging::Config = config
            .focus(crate::logging::LOGGING_CONFIG_KEY)
            .extract()
            .map_err(|e| anyhow!("Failed to load configuration for {}. {}", name, e))?;

        crate::logging::load_logging(name, logging_config);

        Ok(App { config })
    }

    pub fn config<'a, T: serde::Deserialize<'a>>(&self, name: &str) -> anyhow::Result<T> {
        self.config
            .focus(name)
            .extract()
            .map_err(|e| anyhow!("Failed to load configuration for {}. {}", name, e))
    }
}
