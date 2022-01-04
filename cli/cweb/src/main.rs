use cd_cli::dialog::DialogProvider;
use cd_cli::prelude::*;
use std::net::SocketAddr;

#[derive(StructOpt, Debug)]
#[structopt(name = env!("CARGO_BIN_NAME"), version = env!("CARGO_PKG_VERSION"), about = env!("CARGO_PKG_DESCRIPTION"))]
pub struct Opts {
    #[structopt(flatten)]
    pub config: cd_cli::config::ConfigOpts,

    #[structopt(flatten)]
    pub dialog: cd_cli::dialog::DialogOpts,

    #[structopt(subcommand)]
    pub cmd: Option<Cmd>,
}

#[derive(StructOpt, Debug)]
pub enum Cmd {
    Patch,
    Reset,
}

#[derive(StructOpt, Debug)]
pub struct PatchOpts {
    pub name: String,

    #[structopt(long)]
    /// Comments entry, if exists
    pub comment: bool,

    #[structopt(long, conflicts_with = "comment")]
    /// Resolves DNS name and redirects to ip address
    pub to_host: Option<String>,
}

#[derive(serde::Deserialize)]
#[serde(tag = "action")]
pub enum PatchAction {
    Comment { name: String },
    ToHost { name: String, target: String },
}

fn main() -> anyhow::Result<()> {
    let source_host = "www.centerdevice.de";
    let target_host = "www-server.staging.staging.aws.internal.cnter.de";

    let opts = Opts::from_args();
    let app = cd_cli::app::App::new(env!("CARGO_BIN_NAME"), opts.config.clone())?;
    let dialog = app.dialog(opts.dialog)?;

    let etc_hosts = std::fs::read_to_string("/etc/hosts").context("failed to read /etc/hosts file")?;
    let pattern = regex::Regex::new(&format!(r#".+({}).*"#, regex::escape(source_host)))?;

    match opts.cmd {
        _ => {}
    };

    let replaced = pattern.replace_all(&etc_hosts, format!("{} $1", target_host.replace('$', "$$")));

    println!("{}", replaced);

    Ok(())
}

fn resolve(host: &str) -> anyhow::Result<SocketAddr> {
    use std::net::ToSocketAddrs;
    let mut ip_addresses = format!("{}:443", host).to_socket_addrs()?;

    ip_addresses.next().ok_or_else(|| anyhow!("Could not resolve hostname"))
}
