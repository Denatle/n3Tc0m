use std::ops::ControlFlow;

use futures_util::{SinkExt, StreamExt};
// we will use tungstenite for websocket client impl (same library as what axum is using)
use tokio_tungstenite::{
    connect_async,
    tungstenite::protocol::Message,
};
use common::api::{CommandResult, Job};

const SERVER: &str = "ws://0.0.0.0:3000/ws";

pub(crate) async fn spawn_client(who: usize) {
    let ws_stream = match connect_async(SERVER).await {
        Ok((stream, response)) => {
            println!("Handshake for client {who} has been completed");
            // This will be the HTTP response, same as with server this is the last moment we
            // can still access HTTP stuff.
            println!("Server response was {response:?}");
            stream
        }
        Err(e) => {
            println!("WebSocket handshake for client {who} failed with {e}!");
            return;
        }
    };

    let (mut sender, mut receiver) = ws_stream.split();

    // //we can ping the server for start
    // sender
    //     .send(Message::Ping("Hello, Server!".into()))
    //     .await
    //     .expect("Can not send!");

    let msg = receiver.next().await.unwrap().unwrap();
    // print message and break if instructed to do so
    let str: Job = serde_json::from_str(&msg.into_text().unwrap()).unwrap();

    println!("{:?}", str.command);

    let command_result = CommandResult {
        output: str.args.join("")
    };

    sender
        .send(Message::Text(serde_json::to_string(&command_result).unwrap()))
        .await
        .expect("Can not send!");
}

/// Function to handle messages we get (with a slight twist that Frame variant is visible
/// since we are working with the underlying tungstenite library directly without axum here).
fn process_message(msg: Message, who: usize) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            println!(">>> {who} got str: {t:?}");
        }
        Message::Binary(d) => {
            println!(">>> {} got {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {} got close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                println!(">>> {who} somehow got close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            println!(">>> {who} got pong with {v:?}");
        }
        Message::Ping(v) => {
            println!(">>> {who} got ping with {v:?}");
        }

        Message::Frame(_) => {
            unreachable!("This is never supposed to happen")
        }
    }
    ControlFlow::Continue(())
}