use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Job {
    pub command: String,
    pub args: Vec<String>
}
#[derive(Deserialize, Serialize, Debug)]
pub struct CommandResult {
    pub output: String,
}
