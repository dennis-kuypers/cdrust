use cd_cli::prelude::*;
use cd_cli::tunnelblick;

#[derive(StructOpt, Debug)]
#[structopt(name = env!("CARGO_BIN_NAME"), version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Opts {
    #[structopt(flatten)]
    pub config: cd_cli::config::ConfigOpts,

    #[structopt(flatten)]
    pub dialog: cd_cli::dialog::DialogOpts,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();
    let app = cd_cli::app::App::new(env!("CARGO_BIN_NAME"), opts.config.clone())?;

    use cd_cli::tunnelblick::TunnelblickProvider;
    let tunnelblick = app.tunnelblick()?;

    let status = cd_tunnelblick::get_status()?;

    let wants_connect = status
        .iter()
        .any(|s| s.name == tunnelblick.name() && s.state != cd_tunnelblick::State::Connected);
    ensure!(
        wants_connect,
        "{} is already connected. Nothing to do.",
        tunnelblick.name()
    );

    let wants_disconnect = status
        .iter()
        .any(|s| s.state != cd_tunnelblick::State::Exiting && s.name != tunnelblick.name());

    if wants_disconnect {
        use cd_cli::dialog::DialogProvider;
        let output = app.dialog(opts.dialog)?;

        ensure!(
            output.confirm("Disconnect other VPN connections?").unwrap_or_default(),
            "User said no."
        );
        cd_tunnelblick::disconnect_all().map_err(|e| anyhow!(e))?;

        info!("waiting for all connections to exit");
        tunnelblick::wait_for_state(std::time::Duration::from_secs(1), 60, |v| {
            ensure!(!v.is_empty(), "No connections to wait for to disconnect");
            Ok(v.iter().all(|c| c.state == cd_tunnelblick::State::Exiting))
        })?;
    }

    let connecting = cd_tunnelblick::connect(tunnelblick.name())
        .map_err(|e| anyhow!(e))?
        .changed;
    if connecting {
        eprint!("Waiting for Duo confirmation...");

        tunnelblick::wait_for_state(std::time::Duration::from_secs(1), 300, |c| {
            let item = c.iter().find(|s| s.name == tunnelblick.name());
            if let Some(v) = item {
                eprint!(".");
                Ok(v.state == cd_tunnelblick::State::Connected)
            } else {
                bail!("No connection named {}", tunnelblick.name());
            }
        })?;

        eprintln!("connected!");
    } else {
        bail!("`{}` is already connected. Nothing to do.", tunnelblick.name());
    }

    Ok(())
}
