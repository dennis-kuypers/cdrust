use crate::prelude::*;
use crate::shell_multiplexer::ShellMultiplexer;

pub const CONFIG_KEY: &str = "tmux";

#[derive(Deserialize)]
pub struct TmuxConfig {
    pub enabled: Option<bool>,
    pub layout: Option<TmuxLayout>,
}

#[derive(StructOpt, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TmuxLayout {
    NoLayout,
    EvenVertical,
    EvenHorizontal,
    MainHorizontal,
    MainVertical,
    Tiled,
}

pub struct Tmux;
impl ShellMultiplexer for Tmux {
    fn multi_exec(&self, cmds: &[super::ShellCommand]) -> anyhow::Error {
        use std::process::Command;

        let cmds = cmds.split_first().ok_or_else(|| anyhow!("No commands provided"));
        if let Err(e) = cmds {
            return e;
        }
        let (first, rest) = cmds.unwrap();

        for cmd in rest {
            if let Err(e) = Command::new("tmux")
                .arg("split-pane")
                .arg(&cmd.program)
                .args(&cmd.args)
                .spawn()
            {
                return anyhow!(e).context("Spawning new process in tmux pane failed");
            }

            if let Err(e) = Command::new("tmux").arg("select-layout").arg("even-vertical").spawn() {
                return anyhow!(e).context("Failed to re-balance tmux layout");
            }
        }

        use std::os::unix::process::CommandExt;
        Command::new(&first.program).args(&first.args).exec().into()
    }
}
