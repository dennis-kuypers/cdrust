//! Support library for CenterDevice CLI apps.
//!
//! It is recommended to `use cd_cli::prelude::*` to get all the basic building blocks.
//!
//! ## Features
//! * `aws` enables aws support
//!
pub mod app;
#[cfg(feature = "aws")]
pub mod aws;
pub mod config;
pub mod dialog;
pub mod output;
#[cfg(feature = "pivotal")]
pub mod pivotal;
pub mod shell_multiplexer;
#[cfg(feature = "tunnelblick")]
pub mod tunnelblick;

mod logging;

pub mod prelude {
    pub use anyhow::{anyhow, bail, ensure, Context};
    pub use derive_new::new;
    pub use serde::{Deserialize, Serialize};
    pub use structopt::StructOpt;
    pub use tracing::{debug, error, info, instrument, trace, warn};

    pub use crate::app::App;
    pub use crate::output::{Output, PlainTextOutput};
}
