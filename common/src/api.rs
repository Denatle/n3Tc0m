use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct CommandResult {
    pub output: String,
}

#[derive(Deserialize, Serialize)]
pub struct Job {
    pub command: String,
    pub args: Vec<String>
}
