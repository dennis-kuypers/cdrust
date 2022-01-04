use crate::config::{load_config, ConfigOpts};
use crate::prelude::*;

pub struct App {
    config: figment::Figment,
}

impl App {
    pub fn new(name: &str, config: ConfigOpts) -> anyhow::Result<Self> {
        let config = load_config(config, name);

        Ok(App { config })
    }

    pub fn config<'a, T: serde::Deserialize<'a>>(&self, name: &str) -> anyhow::Result<T> {
        self.config.focus(name).extract().map_err(|e| anyhow!(e))
    }
}
