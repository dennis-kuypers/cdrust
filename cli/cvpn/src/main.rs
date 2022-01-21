use cd_cli::prelude::*;
use std::time::Duration;

#[derive(StructOpt, Debug)]
#[structopt(name = env!("CARGO_BIN_NAME"), version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Opts {
    #[structopt(flatten)]
    pub config: cd_cli::config::ConfigOpts,

    #[structopt(flatten)]
    pub dialog: cd_cli::dialog::DialogOpts,
}

// TODO: Move tunnelblick cli specifics to its own lib
const TUNNELBLICK_CONFIG: &str = "tunnelblick";

#[derive(Deserialize)]
pub struct TunnelblickConfig {
    pub connection: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opts = Opts::from_args();
    let app = cd_cli::app::App::new(env!("CARGO_BIN_NAME"), opts.config.clone())?;

    let cfg: TunnelblickConfig = app.config(TUNNELBLICK_CONFIG)?;
    let name = cfg
        .connection
        .ok_or_else(|| anyhow!("No VPN name specified. Configure `vpn.name`"))?;

    let status = cd_tunnelblick::get_status()?;

    let wants_connect = status
        .iter()
        .any(|s| s.name == name && s.state != cd_tunnelblick::State::Connected);
    ensure!(wants_connect, "{} is already connected. Nothing to do.", name);

    let wants_disconnect = status
        .iter()
        .any(|s| s.state != cd_tunnelblick::State::Exiting && s.name != name);

    use cd_cli::dialog::DialogProvider;
    let output = app.dialog(opts.dialog)?;
    if wants_disconnect {
        ensure!(
            output.confirm("Disconnect other VPN connections?").unwrap_or_default(),
            "User said no."
        );
        cd_tunnelblick::disconnect_all().map_err(|e| anyhow!(e))?;

        info!("waiting for all connections to exit");
        wait_for_connection(Duration::from_secs(1), 60, |v| {
            ensure!(!v.is_empty(), "No connections to wait for to disconnect");
            Ok(v.iter().all(|c| c.state == cd_tunnelblick::State::Exiting))
        })?;
    }

    let connecting = cd_tunnelblick::connect(&name).map_err(|e| anyhow!(e))?.changed;
    if connecting {
        eprint!("Waiting for Duo confirmation...");

        wait_for_connection(Duration::from_secs(1), 300, |c| {
            let item = c.iter().find(|s| s.name == name);
            if let Some(v) = item {
                eprint!(".");
                Ok(v.state == cd_tunnelblick::State::Connected)
            } else {
                bail!("No connection named {}", name);
            }
        })?;

        eprintln!("connected!");
    } else {
        bail!("`{}` is already connected. Nothing to do.", name);
    }

    Ok(())
}

fn wait_for_connection<F>(wait: Duration, retries: u32, f: F) -> anyhow::Result<bool>
where
    F: Fn(Vec<cd_tunnelblick::Vpn>) -> anyhow::Result<bool>,
{
    for _ in 1..=retries {
        let status = cd_tunnelblick::get_status()?;
        match f(status) {
            Ok(false) => std::thread::sleep(wait),
            failure_or_success => return failure_or_success,
        }
    }

    Ok(false)
}
