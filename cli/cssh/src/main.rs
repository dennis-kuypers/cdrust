//! # cssh
//!
//!

use cd_cli::prelude::*;

#[derive(StructOpt, Debug)]
#[structopt(name = env!("CARGO_BIN_NAME"), version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Opts {
    #[structopt(flatten)]
    pub config: cd_cli::config::ConfigOpts,

    #[structopt(flatten)]
    pub select: cd_cli::aws::ec2::Ec2SelectOpt,

    #[structopt(flatten)]
    pub user: cd_cli::dialog::DialogOpts,
}

static CONFIG_NAME: &str = "cssh";

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    /// Selection mode to be used when no filters are given
    #[serde(default)]
    pub default_selection: SelectionMode,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SelectionMode {
    Single,
    Multi,
}

impl Default for SelectionMode {
    fn default() -> Self {
        SelectionMode::Single
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use cd_cli::shell_multiplexer::*;

    let opts: Opts = Opts::from_args();
    let app = App::new(env!("CARGO_BIN_NAME"), opts.config)?;
    let config: Config = app.config(CONFIG_NAME)?;

    use cd_cli::aws::AwsProvider;
    let aws_client = app.aws_client().await?;

    let instances = cd_cli::aws::ec2::select_instances(&opts.select, &aws_client).await?;

    ensure!(!instances.is_empty(), "search returned no results");

    if let [target] = instances.as_slice() {
        // If we only found a single host we can launch right away
        info!("Found a single instance: {}, connecting...", target.to_short_string());

        return Err(ShellCommand::new("ssh".to_string(), vec![format!("cd-admin@{}", target.private_ip)]).exec());
    }

    use cd_cli::shell_multiplexer::ShellMultiplexerProvider;
    let multi_exec = app.shell_multiplexer()?;

    use cd_cli::dialog::DialogProvider;
    let interactive = app.dialog(opts.user)?;

    let no_multiplexer_support = multi_exec.is_none();
    eprintln!("{:?}, {:?}", opts.select.has_no_filters(), config.default_selection);
    if no_multiplexer_support || (opts.select.has_no_filters() && config.default_selection == SelectionMode::Single) {
        let instance = cd_cli::aws::dialog::select_by_user_selection(instances, &interactive, "select ssh target")?;
        let ssh_command = ShellCommand::new("ssh".to_string(), vec![format!("cd-admin@{}", instance.private_ip)]);
        return Err(ssh_command.exec());
    }

    // unwrap: Safe, because we checked before for 'single' execute branch
    let multi_exec = multi_exec.unwrap();

    let instances =
        cd_cli::aws::dialog::filter_by_user_selection(instances, &interactive, "Select machines to ssh into", true)?;

    let commands: Vec<_> = instances
        .into_iter()
        .map(|i| ShellCommand::new("ssh".to_string(), vec![format!("cd-admin@{}", i.private_ip)]))
        .collect();

    Err(multi_exec.multi_exec(&commands))
}
