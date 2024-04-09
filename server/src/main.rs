use std::{
    net::SocketAddr,
    ops::ControlFlow,
};

use axum::{
    extract::ConnectInfo,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    Router, routing::get,
};
// for socket split
// use futures::SinkExt;

use common::api::Job;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ws", get(handler));


    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

async fn handler(ws: WebSocketUpgrade, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(mut socket: WebSocket, who: SocketAddr) {
    // if socket.send(Message::Ping(vec![1, 2, 3])).await.is_ok() {
    //     println!("Pinged {who}...");
    // } else {
    //     println!("Could not send ping {who}!");
    //     return;
    // }

    let new_job: Job = Job {
        command: "echo".into(),
        args: vec!["hello_world".into()],
    };

    // send job
    if socket
        .send(Message::Text(serde_json::to_string(&new_job).unwrap()))
        .await
        .is_err()
    {
        println!("client {who} abruptly disconnected");
        return;
    }

    // receive job
    if let Some(msg) = socket.recv().await {
        if let Ok(msg) = msg {
            if process_message(msg, who).is_break() {
                return;
            }
        } else {
            println!("client {who} abruptly disconnected");
            return;
        }
    }
    println!("Websocket context {who} destroyed");
}

/// helper to print contents of messages to stdout. Has special treatment for Close.
fn process_message(msg: Message, who: SocketAddr) -> ControlFlow<(), ()> {
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