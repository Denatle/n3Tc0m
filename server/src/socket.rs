use std::net::SocketAddr;

use axum::extract::{ConnectInfo, WebSocketUpgrade};
use axum::extract::ws::WebSocket;
use axum::response::IntoResponse;
use futures::StreamExt;
use common::api::Job;

use crate::employer::{employ, execute_job};

pub(crate) async fn handler(ws: WebSocketUpgrade, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, addr))
}

async fn handle_socket(socket: WebSocket, who: SocketAddr) {
    let (sender, receiver) = socket.split();

    //TODO Some info before execution

    employ(who, sender.reunite(receiver).unwrap()).await;
    let job = Job {
        args: vec!["hello world".into()],
        command: "echo".into(),
    };
    let _ = execute_job(who, job).await;
}
