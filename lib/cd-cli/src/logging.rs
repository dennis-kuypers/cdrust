use tracing_subscriber::EnvFilter;

pub static LOGGING_CONFIG_KEY: &str = "log";

#[derive(serde::Deserialize)]
pub struct Config {
    pub custom: Option<String>,
}

pub fn load_logging(name: &str, config: Config) {
    // prefer env var based on binary name
    let env_name = format!("{}_LOG", name.replace('-', "_").to_uppercase());
    let env_filter = EnvFilter::try_from_env(env_name).ok();

    // then use config
    let config_value = config.custom.filter(|t| !t.is_empty());
    let config_filter = config_value.and_then(|c| EnvFilter::try_from(c).ok());

    let selected_env_filter = env_filter.or(config_filter).unwrap_or_default();

    tracing_subscriber::FmtSubscriber::builder()
        .with_max_level(tracing::Level::ERROR)
        .with_env_filter(selected_env_filter)
        .with_writer(std::io::stderr)
        .init()
}
