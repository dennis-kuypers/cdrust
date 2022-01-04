//! haproxy-agent implements an agent for haproxy's `agent-check` directive.
//!
//! * haproxy [agent-check](http://cbonte.github.io/haproxy-dconv/2.4/configuration.html#5.2-agent-check)
//!
//! # haproxy config
//! Point haproxy at this service. This agent will query systemd via dbus to get the service status.
//!
//! Note: _Non-existent services may show up as `Stopped`_
//!
//! agent-check - Enables agent health checks for the server.
//! agent-addr  - Identifies the IP address where the agent is listening.
//! agent-port  - Identifies the port where the agent is listening.
//! agent-inter - Defines the interval between health checks.
//! agent-send  - A string that HAProxy sends to the agent upon connection. Be sure to end it with a newline character.
//!
//! ## Example
//! `agent-check agent-addr 127.0.0.1 agent-port 13370 agent-send servicename\n`
mod config;
mod server;

use crate::server::ServerParameters;
use std::time::Duration;
use tracing::*;

fn main() {
    use tracing_subscriber::{EnvFilter, FmtSubscriber};

    FmtSubscriber::builder()
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let config = config::load();

    let systemd = cd_systemd::SystemdConnection::new_system();
    if let Err(e) = systemd {
        error!("systemd (dbus) connectivity is required to function: {}", e);
        std::process::exit(exitcode::UNAVAILABLE);
    }
    let systemd = systemd.unwrap();

    let max_clients = config.max_clients.unwrap_or(100);
    let client_timeout = Duration::from_millis(config.client_timeout.unwrap_or(2000) as u64);

    let mut serverparams = server::ServerParameters {
        max_clients,
        client_timeout,
        ..server::ServerParameters::default()
    };

    if let Some(addr) = config.bind {
        let listen_addr: Result<std::net::SocketAddr, _> = addr.parse();
        match listen_addr {
            Err(e) => {
                error!("Can not parse bind address: {}", e);
                std::process::exit(exitcode::CONFIG);
            }
            Ok(listen_addr) => {
                serverparams = ServerParameters {
                    listen_addr,
                    ..serverparams
                }
            }
        }
    }

    server::serve(systemd, serverparams);
}
