use aws_sdk_ec2::model::{Instance, InstanceState, InstanceStateChange};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::ops::BitAnd;
use thiserror::Error;

#[derive(Serialize)]
pub struct Ec2Instance {
    pub id: String,
    pub state: Ec2InstanceState,
    pub private_ip: String,
    pub private_dns: String,
    pub tags: HashMap<String, String>,
}

impl Ec2Instance {
    pub fn to_short_string(&self) -> String {
        let name = self.tags.get("Name").map(|s| s.as_str()).unwrap_or("");
        format!("{} ({}, {:?})", name, self.id, self.state)
    }
}

impl TryFrom<aws_sdk_ec2::model::Instance> for Ec2Instance {
    type Error = ParseError;

    fn try_from(value: Instance) -> Result<Self, Self::Error> {
        let instance = Ec2Instance {
            id: value.instance_id.ok_or_else(|| ParseError("instance_id".into()))?,
            state: value.state.ok_or_else(|| ParseError("state".into()))?.into(),
            private_ip: value
                .private_ip_address
                .ok_or_else(|| ParseError("ip_private".into()))?,
            private_dns: value.private_dns_name.ok_or_else(|| ParseError("ip_private".into()))?,
            tags: value
                .tags
                .unwrap_or_default()
                .into_iter()
                .map(|v| (v.key.unwrap_or_default(), v.value.unwrap_or_default()))
                .collect(),
        };
        Ok(instance)
    }
}

#[derive(Serialize, Debug)]
pub enum Ec2InstanceState {
    Unknown,
    Pending,
    Running,
    ShuttingDown,
    Terminated,
    Stopping,
    Stopped,
}

impl From<aws_sdk_ec2::model::InstanceState> for Ec2InstanceState {
    // Lower bits
    //  0 : pending
    // 16 : running
    // 32 : shutting-down
    // 48 : terminated
    // 64 : stopping
    // 80 : stopped
    fn from(s: InstanceState) -> Self {
        match s.code.unwrap_or(0).bitand(0xFF) {
            0 => Ec2InstanceState::Pending,
            16 => Ec2InstanceState::Running,
            32 => Ec2InstanceState::ShuttingDown,
            48 => Ec2InstanceState::Terminated,
            64 => Ec2InstanceState::Stopping,
            80 => Ec2InstanceState::Stopped,
            _ => Ec2InstanceState::Unknown,
        }
    }
}

impl Default for Ec2InstanceState {
    fn default() -> Self {
        Self::Unknown
    }
}

impl Display for Ec2InstanceState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Ec2InstanceState::Pending => "stopping",
            Ec2InstanceState::Running => "running",
            Ec2InstanceState::ShuttingDown => "shutting down",
            Ec2InstanceState::Terminated => "terminated",
            Ec2InstanceState::Stopping => "stopping",
            Ec2InstanceState::Stopped => "stopped",
            Ec2InstanceState::Unknown => "unknown",
        };
        f.write_str(name)
    }
}

#[derive(Serialize, Debug)]
pub struct Ec2InstanceStateChange {
    pub instance_id: String,
    pub previous_state: Ec2InstanceState,
    pub current_state: Ec2InstanceState,
}

impl From<InstanceStateChange> for Ec2InstanceStateChange {
    fn from(i: InstanceStateChange) -> Self {
        Self {
            instance_id: i.instance_id.unwrap_or_default(),
            previous_state: i.previous_state.map(|i| i.into()).unwrap_or_default(),
            current_state: i.current_state.map(|i| i.into()).unwrap_or_default(),
        }
    }
}

#[derive(Error, Debug)]
#[error("Missing field on {0}")]
pub struct ParseError(String);
