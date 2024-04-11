use std::collections::HashMap;
use std::net::SocketAddr;

use axum::extract::ws::{Message, WebSocket};
use futures::StreamExt;
use lazy_static::lazy_static;
use tokio::sync::{Mutex, MutexGuard, RwLock};

use common::api::{CommandOutput, CommandResult, Job};
use common::errors::CommandErrors;
use common::errors::CommandErrors::*;

lazy_static! {
    static ref AGENTS: RwLock<HashMap<SocketAddr, Mutex<WebSocket>>> = RwLock::new(HashMap::new());
}

pub async fn employ(socket_addr: SocketAddr, web_socket: WebSocket) {
    AGENTS.write().await.insert(socket_addr, Mutex::from(web_socket));
    info!("Employed {socket_addr}")
}

pub async fn get_agents() -> Vec<SocketAddr> {
    let agents_read = AGENTS
        .read().await;
    agents_read.keys().cloned().collect()
}

pub async fn execute_job(socket_addr: SocketAddr, job: Job) -> CommandResult {
    info!("Executing job ({:?}) on {socket_addr}", job);
    let message_text = match execute(socket_addr, job).await {
        CommandResult::CommandOutput(o) => { o.output }
        CommandResult::CommandError(e) => {
            return match e {
                ConnectionClosed => {
                    fire(socket_addr).await;
                    CommandResult::CommandError(ConnectionClosed)
                }
                e => { CommandResult::CommandError(e) }
            };
        }
    };

    let result: CommandResult = match serde_json::from_str::<CommandResult>(&message_text) {
        Ok(r) => { r }
        Err(_) => { return CommandResult::CommandError(BadMessage); }
    };


    info!("Job ({socket_addr}) resulted in {:?}", result);
    result
}

async fn fire(socket_addr: SocketAddr) {
    let mut agents_write = AGENTS
        .write().await;
    agents_write.remove(&socket_addr);
    info!("Fired {socket_addr}");
}

async fn execute(socket_addr: SocketAddr, job: Job) -> CommandResult {
    let agents_read = AGENTS
        .read().await;
    let socket = match agents_read
        .get(&socket_addr) {
        Some(r) => {r.lock().await},
        None => {
            drop(agents_read);
            fire(socket_addr).await;
            info!("returned");
            return CommandResult::CommandError(ConnectionClosed)
        },
    };
        
    let message = match send_job(socket, job).await {
        Some(m) => { m }
        None => {
            fire(socket_addr).await;
            return CommandResult::CommandError(NoMessage);
        }
    };
    let message_text = match process_message(message).await {
        Ok(text) => { text }
        Err(e) => {
            return match e {
                ConnectionClosed => {
                    fire(socket_addr).await;
                    CommandResult::CommandError(ConnectionClosed)
                }
                e => { CommandResult::CommandError(e) }
            };
        }
    };
    CommandResult::CommandOutput(CommandOutput {
        output: message_text
    })
}

async fn send_job(mut socket: MutexGuard<'_, WebSocket>, job: Job) -> Option<Message> {
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

async fn process_message(message: Message) -> Result<String, CommandErrors> {
    match message {
        Message::Text(m) => { Ok(m) }
        Message::Close(_) => { Err(ConnectionClosed) }
        _ => { Err(TypeNotSupported) }
    }
}
