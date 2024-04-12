use tokio::process::Command;
use common::api::{CommandOutput, CommandPayload, JobOutput};

pub async fn execute_command(p: &CommandPayload) -> JobOutput {
    let result = Command::new(&p.command).args(&p.args).output().await;
    let output: String = match result {
        Ok(res) => String::from_utf8(res.stdout).unwrap(),
        Err(err) => {
            err.to_string()
        }
    };

    JobOutput::Command(Ok(CommandOutput { output }))
}