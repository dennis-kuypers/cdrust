use crate::prelude::*;
pub mod tmux;

pub trait ShellMultiplexerProvider {
    fn shell_multiplexer(&self) -> anyhow::Result<Option<Box<dyn ShellMultiplexer>>>;
}

impl ShellMultiplexerProvider for App {
    fn shell_multiplexer(&self) -> anyhow::Result<Option<Box<dyn ShellMultiplexer>>> {
        let config: tmux::TmuxConfig = self.config(tmux::CONFIG_KEY)?;
        if !config.enabled.unwrap_or_default() {
            return Ok(None);
        }

        Ok(Some(Box::new(tmux::Tmux)))
    }
}
pub trait ShellMultiplexer {
    fn multi_exec(&self, cmds: &[ShellCommand]) -> anyhow::Error;
}

#[derive(new)]
pub struct ShellCommand {
    program: String,
    args: Vec<String>,
}

impl ShellCommand {
    pub fn spawn(&self) -> anyhow::Result<()> {
        std::process::Command::new(&self.program)
            .args(&self.args)
            .spawn()
            .map(|_| ())
            .map_err(|e| anyhow!(e))
    }

    pub fn exec(&self) -> anyhow::Error {
        use std::os::unix::process::CommandExt;

        std::process::Command::new(&self.program).args(&self.args).exec().into()
    }
}

impl From<ShellCommand> for std::process::Command {
    fn from(sc: ShellCommand) -> Self {
        let mut cmd = std::process::Command::new(sc.program);
        cmd.args(sc.args);
        cmd
    }
}
