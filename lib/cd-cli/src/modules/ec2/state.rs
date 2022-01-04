use crate::prelude::*;
use std::fmt::{Display, Formatter};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct StateOpts {
    #[structopt(flatten)]
    pub select: crate::modules::ec2::Ec2SelectOpt,
}

pub enum State {
    Start,
    Stop,
    Terminate,
}

impl Display for State {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Start => f.write_str("start"),
            State::Stop => f.write_str("stop"),
            State::Terminate => f.write_str("terminate"),
        }
    }
}

pub async fn change_state(
    opts: StateOpts,
    client: &ce_aws::AwsClient,
    state: State,
    output: Output,
) -> Result<(), anyhow::Error> {
    // acquire output first, so we don't query API if interactivity is now allowed (we need confirmation)
    let output = output.dialog()?;

    let instances = super::select_instances(&opts.select, &client).await?;
    let instances = super::filter_by_user_selection(instances, &output, format!("Select Instances to {}", state))?;
    let instance_ids = instances.into_iter().map(|i| i.id).collect();

    let output = output.plaintext()?;

    use ce_aws::ec2::{start_instances, stop_instances, terminate_instances};
    let state_changes = match state {
        State::Start => start_instances(&client, instance_ids).await?,
        State::Stop => stop_instances(&client, instance_ids).await?,
        State::Terminate => terminate_instances(&client, instance_ids).await?,
    };

    for state_change in state_changes {
        output.println(&format!(
            "{} ({}->{})",
            state_change.instance_id, state_change.previous_state, state_change.current_state
        ));
    }

    Ok(())
}
