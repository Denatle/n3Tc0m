use std::error::Error;
use std::process::Output;
use std::time::Duration;
use reqwest::{Client, StatusCode};
use tokio::process::Command;
use common::api::{CommandResult, Job};

const POLL_RATE: Duration = Duration::from_millis(500);

pub(super) async fn run() -> Result<(), reqwest::Error> {
    let client = Client::new();
    loop {
        let job = poll(&client).await;
        let result = match job {
            Ok(job) => execute(job).await,
            Err(_) => continue
        };
        let output = match result {
            Ok(res) => res,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
        let code = send_output(&client, output).await;
        match code {
            Ok(code) => println!("{:#?}", code),
            Err(err) => println!("{:#?}", err)
        }
    }
}

async fn poll(client: &Client) -> Result<Job, reqwest::Error> {
    let job = loop {
        tokio::time::sleep(POLL_RATE).await;
        let response = client.get("http://0.0.0.0:3000/jobs").send().await;
        match response {
            Ok(..) => break response?,
            Err(_) => continue
        };
    };
    let job = job.json::<Job>().await?;
    Ok(job)
}

async fn execute(job: Job) -> Result<Output, Box<dyn Error>> {
    let output = Command::new(job.command).args(&job.args).output().await?;
    Ok(output)
}

async fn send_output(client: &Client, output: Output) -> Result<StatusCode, reqwest::Error> {
    let result = CommandResult {
        output: String::from_utf8(output.stdout).unwrap(),
    };
    let res = client.post("http://0.0.0.0:3000/data").json(&result).send().await?;
    Ok(res.status())
}