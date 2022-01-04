use crate::prelude::*;

pub fn load_config(opts: ConfigOpts, name: &str) -> figment::Figment {
    use figment::{
        providers::{Env, Format, Toml},
        Figment, Profile,
    };
    let user_dir = directories::UserDirs::new()
        .expect("expect user directories to be detectable")
        .home_dir()
        .join(".cd");

    let profile_file_name = user_dir.join("profile");
    let profile = opts
        .profile
        .or_else(|| {
            std::fs::read_to_string(profile_file_name)
                .map(|s| s.trim().to_string())
                .ok()
        })
        .expect("No profile provided (create profile file in ~/.cd/profile or use flag -p)");

    info!("Active Profile: {}", profile);

    let mut figment = Figment::new();

    figment = figment.merge(Toml::file(user_dir.join("default.conf")).profile(Profile::Default));
    figment = figment.merge(Toml::file(user_dir.join(format!("{}.default.conf", name))).profile(Profile::Default));

    figment = figment.merge(Toml::file(user_dir.join(format!("{}.conf", profile))).profile(&profile));
    figment = figment.merge(Toml::file(user_dir.join(format!("{}.conf", name))).nested());

    figment = figment.merge(Toml::file(user_dir.join("global.conf")).profile(Profile::Global));
    figment = figment.merge(Toml::file(user_dir.join(format!("{}.global.conf", name))).profile(Profile::Global));

    // if let Some(file) = opts.config {
    //     debug!("Loading config file `{}` due to command line override", file);
    //     figment = figment.merge(Toml::file(file).nested());
    // }

    let env_prefix = format!("{}_", name.to_uppercase());
    debug!("loading env variables using prefix `{}`", env_prefix);
    figment = figment.merge(Env::prefixed(&env_prefix));

    if opts.dump {
        eprintln!("CONFIG DUMP:\n{:?}", figment);
    }
    figment.select(profile)
}

#[derive(StructOpt, Debug, Clone)]
pub struct ConfigOpts {
    #[structopt(short, long)]
    /// The profile to load
    pub profile: Option<String>,

    #[structopt(long)]
    /// Ignore default files and only load given file
    pub config: Option<String>,

    #[structopt(long = "config-dump")]
    /// Dump configuration to stderr
    pub dump: bool,
}
