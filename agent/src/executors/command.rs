use tokio::process::Command;
use common::api::{CommandOutput, CommandPayload, JobOutput};

pub async fn execute_command(p: &CommandPayload) -> JobOutput {
    #[cfg(target_os = "windows")]
    let mut new_command = vec!["-c".to_string(), p.command.clone()];
    #[cfg(target_os = "windows")]
    new_command.append(&mut p.args.clone());
    #[cfg(target_os = "windows")]
    let result = Command::new("powershell.exe")
        .env("OutputEncoding", "[console]::InputEncoding = [console]::OutputEncoding = New-Object System.Text.UTF8Encoding")
        .args(new_command)
        .output()
        .await;
    #[cfg(target_os = "linux")]
    let result = Command::new(&p.command).args(&p.args).output().await;

    let output: String = match result {
        Ok(res) => {
            String::from_utf8_lossy(&res.stdout).parse().unwrap()
        }
        Err(err) => {
            err.to_string()
        }
    };

    JobOutput::Command(Ok(CommandOutput { output }))
}