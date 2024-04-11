use std::process::Output;

use futures_util::{SinkExt, StreamExt};
use futures_util::stream::{SplitSink, SplitStream};
use tokio::net::TcpStream;
use tokio::process::Command;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};
use tokio_tungstenite::tungstenite::{Error, Message};

use common::api::{CommandResult, Job};

pub(super) async fn run(socket: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Result<(), Error> {
    let (mut sender, mut receiver) = socket.split();
    loop {
        let job = receive(&mut receiver).await?;
        let result = execute(job).await;

        let output: String = match result {
            Ok(res) => String::from_utf8(res.stdout).unwrap(),
            Err(err) => {
                err.to_string()
            }
        };
        send_output(&mut sender, output).await?;
    }
}

async fn receive(receiver: &mut SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>) -> Result<Job, Error> {
    let message = match receiver.next().await {
        Some(message) => message?,
        None => return Err(Error::ConnectionClosed),
    };

    let job = match process_message(message).await {
        Ok(message) => { message }
        Err(e) => {
            return match e {
                Error::ConnectionClosed => { Err(Error::ConnectionClosed) }
                Error::Utf8 => { Err(Error::Utf8) }
                _ => { unreachable!("Wrong error") }
            };
        }
    };

    let job: Job = serde_json::from_str(&job).unwrap();
    Ok(job)
}

async fn execute(job: Job) -> Result<Output, tokio::io::Error> {
    let output = Command::new(job.command).args(&job.args).output().await?;
    Ok(output)
}

async fn send_output(sender: &mut SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>, output: String) -> Result<(), Error> {
    let result = CommandResult { output };
    sender.send(Message::Text(serde_json::to_string(&result).unwrap())).await?;
    Ok(())
}

async fn process_message(msg: Message) -> Result<String, Error> {
    match msg {
        Message::Text(t) => {
            Ok(t)
        }
        Message::Close(_) => {
            Err(Error::ConnectionClosed)
        }
        Message::Frame(_) => {
            unreachable!("This is never supposed to happen")
        }
        _ => { Err(Error::Utf8) }
    }
}