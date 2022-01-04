use std::path::Path;
use tracing::debug;

pub fn load_config(name: &str) -> figment::Figment {
    use figment::{
        providers::{Env, Format, Toml},
        Figment, Profile,
    };

    let mut figment = Figment::new();

    let etc_file = Path::new("/etc").join(format!("{}.conf", name));
    debug!("loading configuration file at {}", etc_file.display());
    figment = figment.merge(Toml::file(etc_file).profile(Profile::Default));

    let etc_d = Path::new("/etc").join(format!("{}.d/", name));
    debug!("scanning for configuration files at {}", etc_d.display());
    if let Ok(files) = std::fs::read_dir(etc_d) {
        for file in files.flatten() {
            figment = figment.merge(Toml::file(file.path()).profile(Profile::Default));
            debug!("loaded configuration file at {}", file.path().display());
        }
    }

    let env_prefix = format!("{}_", name.replace('-', "_").to_uppercase());
    debug!("loading env variables using prefix `{}`", env_prefix);
    figment = figment.merge(Env::prefixed(&env_prefix));

    figment.select(Profile::Default)
}
