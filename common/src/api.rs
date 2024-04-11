use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use crate::errors::CommandErrors;

#[derive(Deserialize, Serialize, Debug)]
pub struct Job {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CommandOutput {
    pub output: String,
}


#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Agents {
    pub socket_addrs: Vec<SocketAddr>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TargetJob {
    pub socket_addr: SocketAddr,
    pub job: Job,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum CommandResult {
    CommandOutput(CommandOutput),
    CommandError(CommandErrors),
}