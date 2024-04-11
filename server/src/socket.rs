use std::borrow::Cow;
use std::net::SocketAddr;
use std::ops::ControlFlow;

use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::extract::ws::{close_code, CloseFrame, Message, WebSocket};
use axum::response::IntoResponse;
use futures::{SinkExt, StreamExt};

use common::api::Job;

pub(crate) async fn handler(ws: WebSocketUpgrade, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(socket: WebSocket, who: SocketAddr) {
    let (mut sender, mut receiver) = socket.split();

    let new_job: Job = Job {
        command: "echo".into(),
        args: vec!["hello_world".into()],
    };

    // send job
    if sender
        .send(Message::Text(serde_json::to_string(&new_job).unwrap()))
        .await
        .is_err()
    {
        println!("client {who} abruptly disconnected");
        return;
    }

    // receive job
    if let Some(msg) = receiver.next().await {
        if let Ok(msg) = msg {
            if process_message(msg, who).await.is_break() {
                return;
            }
        } else {
            println!("client {who} abruptly disconnected");
            return;
        }
    }

    if let Err(e) = sender
        .send(Message::Close(Some(CloseFrame {
            code: close_code::NORMAL,
            reason: Cow::from("Need to, sry"),
        })))
        .await {
        println!("Could not send Close due to {e:?}, probably it is ok?");
    };
    
    println!("Websocket context {who} destroyed");
}

/// helper to print contents of messages to stdout. Has special treatment for Close.
async fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
    match msg {
        Message::Text(t) => {
            println!(">>> {who} sent str: {t:?}");
        }
        Message::Binary(d) => {
            println!(">>> {} sent {} bytes: {:?}", who, d.len(), d);
        }
        Message::Close(c) => {
            if let Some(cf) = c {
                println!(
                    ">>> {} sent close with code {} and reason `{}`",
                    who, cf.code, cf.reason
                );
            } else {
                println!(">>> {who} somehow sent close message without CloseFrame");
            }
            return ControlFlow::Break(());
        }

        Message::Pong(v) => {
            println!(">>> {who} sent pong with {v:?}");
        }

        Message::Ping(_) => {}
    }
    ControlFlow::Continue(())
}
