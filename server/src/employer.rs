use std::collections::HashMap;
use std::net::SocketAddr;

use axum::extract::ws::{Message, WebSocket};
use futures::{StreamExt};
use lazy_static::lazy_static;
use tokio::sync::{Mutex, MutexGuard, RwLock};

use common::api::{CommandResult, Job};

use crate::errors::ComErrors;
use crate::errors::ComErrors::ConnectionClosed;

lazy_static! {
    static ref AGENTS: RwLock<HashMap<SocketAddr, Mutex<WebSocket>>> = RwLock::new(HashMap::new());
}

pub async fn employ(socket_addr: SocketAddr, web_socket: WebSocket) {
    AGENTS.write().await.insert(socket_addr, Mutex::from(web_socket));
    info!("Employed {socket_addr}")
}

pub async fn execute_job(socket_addr: SocketAddr, job: Job) -> Result<CommandResult, ComErrors> {
    info!("Executing job ({:?}) on {socket_addr}", job);
    let agents_read = AGENTS
        .read().await;
    let socket = agents_read
        .get(&socket_addr)
        .unwrap()
        .lock().await;

    let message = match execute(socket, job).await {
        Some(m) => { m }
        None => { return Err(ComErrors::NoMessage); }
    };
    let message_text = match process_message(message).await {
        Ok(text) => { text }
        Err(e) => {
            return match e {
                ConnectionClosed => {
                    fire(socket_addr).await;
                    Err(ConnectionClosed)
                }
                e => { Err(e) }
            };
        }
    };
    let result: CommandResult = serde_json::from_str(&message_text).unwrap();
    info!("Job ({socket_addr}) resulted in {:?}", result);
    Ok(result)
}

async fn fire(socket_addr: SocketAddr) {
    let mut agents_write = AGENTS
        .write().await;
    agents_write.remove(&socket_addr);
    info!("Fired {socket_addr}");
}

async fn execute(mut socket: MutexGuard<'_, WebSocket>, job: Job) -> Option<Message> {
    if socket
        .send(Message::Text(serde_json::to_string(&job).unwrap()))
        .await
        .is_err()
    {
        warn!("Client abruptly disconnected");
        return None;
    }

    let result = socket.next().await.unwrap().unwrap();
    Some(result)
}

async fn process_message(message: Message) -> Result<String, ComErrors> {
    match message {
        Message::Text(m) => { Ok(m) }
        Message::Close(_) => { Err(ConnectionClosed) }
        _ => { Err(ComErrors::TypeNotSupported) }
    }
}
