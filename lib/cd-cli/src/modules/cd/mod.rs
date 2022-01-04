// use crate::prelude::*;
use structopt::StructOpt;
mod health;

#[derive(StructOpt, Debug)]
/// Interacts with centerdevice
pub struct Opts {
    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(alias = "status")]
    /// Performs a health check
    Health(health::Opts),
}
