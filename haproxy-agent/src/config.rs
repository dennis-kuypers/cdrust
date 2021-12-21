use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub bind: Option<String>,
    pub max_clients: Option<usize>,
    pub client_timeout: Option<u32>,
}

pub(crate) fn load() -> Config {
    let config = cd_service_config::load_config(env!("CARGO_PKG_NAME")).extract();
    if let Err(e) = config {
        tracing::error!("Config load error: {}", e);
        std::process::exit(exitcode::CONFIG);
    };
    config.unwrap()
}
