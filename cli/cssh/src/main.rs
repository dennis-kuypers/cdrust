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

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    use cd_cli::shell_multiplexer::*;

    let opts = Opts::from_args();
    let app = App::new(env!("CARGO_BIN_NAME"), opts.config)?;

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

    if let Some(me) = multi_exec {
        let select_by_default = opts.select.query.is_some();

        // tmux can do multiple instances, allow multi-selection
        let instances = cd_cli::aws::dialog::filter_by_user_selection(
            instances,
            &interactive,
            "Select machines to ssh into",
            select_by_default,
        )?;

        let sshs: Vec<_> = instances
            .into_iter()
            .map(|i| ShellCommand::new("ssh".to_string(), vec![format!("cd-admin@{}", i.private_ip)]))
            .collect();

        Err(me.multi_exec(&sshs))
    } else {
        let instance =
            cd_cli::aws::dialog::select_by_user_selection(instances, &interactive, "Select machine to ssh into.")?;
        let ssh_command = ShellCommand::new("ssh".to_string(), vec![format!("cd-admin@{}", instance.private_ip)]);
        Err(ssh_command.exec())
    }
}
