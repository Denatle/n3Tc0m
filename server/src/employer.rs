use std::collections::HashMap;
use std::net::SocketAddr;

use axum::extract::ws::{Message, WebSocket};
use futures::StreamExt;
use lazy_static::lazy_static;
use serde_json::from_str;
use tokio::sync::{Mutex, MutexGuard, RwLock};

use common::api::{Job, JobResult, TargetJob};
use common::errors::JobErrors;
use common::errors::JobErrors::*;

type Agents = HashMap<SocketAddr, (Mutex<WebSocket>, RwLock<HashMap<u16, Job>>)>;

lazy_static! {
    static ref AGENTS: RwLock<Agents> = RwLock::new(HashMap::new());
}

// TODO: Job ids and cancelling

pub async fn employ(socket_addr: SocketAddr, web_socket: WebSocket) {
    AGENTS.write().await.insert(socket_addr, (Mutex::from(web_socket), RwLock::new(HashMap::new())));
    info!("Employed {socket_addr}")
}

pub async fn get_agents() -> Vec<SocketAddr> {
    let agents_read = AGENTS
        .read().await;
    agents_read.keys().cloned().collect()
}

pub async fn execute_job(socket_addr: SocketAddr, target_job: TargetJob) -> JobResult {
    info!("Executing job ({:?}) on {socket_addr}", target_job);
    let job = Job {
        job_type: target_job.job_type,
        id: 0,
        timeout: target_job.timeout,
    };
    let result = execute(socket_addr, job).await;

    info!("Job ({socket_addr}, {}) resulted in {:?}", result.id, serde_json::to_string(&result.job_output).unwrap());
    result
}

async fn fire(socket_addr: SocketAddr) {
    let mut agents_write = AGENTS
        .write().await;
    agents_write.remove(&socket_addr);
    info!("Fired {socket_addr}");
}

async fn execute(socket_addr: SocketAddr, job: Job) -> JobResult {
    let agents_read = AGENTS
        .read().await;
    let socket = match agents_read
        .get(&socket_addr) {
        Some(r) => { r.0.lock().await }
        None => {
            drop(agents_read);
            fire(socket_addr).await;
            return JobResult { id: job.id, job_output: Err(SocketNotFound) };
        }
    };

    let message = match send_job(socket, &job).await {
        Some(m) => { m }
        None => {
            drop(agents_read);
            fire(socket_addr).await;
            return JobResult { id: job.id, job_output: Err(NoMessage) };
        }
    };
    let message_text = match process_message(message).await {
        Ok(text) => { text }
        Err(e) => {
            return match e {
                ConnectionClosed => {
                    drop(agents_read);
                    fire(socket_addr).await;
                    JobResult { id: job.id, job_output: Err(ConnectionClosed) }
                }
                e => { JobResult { id: job.id, job_output: Err(e) } }
            };
        }
    };

    match from_str::<JobResult>(&message_text) {
        Ok(r) => { r }
        Err(_) => { JobResult { id: job.id, job_output: Err(BadAgentMessage) } }
    }
}

async fn send_job(mut socket: MutexGuard<'_, WebSocket>, job: &Job) -> Option<Message> {
    if socket
        .send(Message::Text(serde_json::to_string(job).unwrap()))
        .await
        .is_err()
    {
        warn!("Agent abruptly disconnected");
        return None;
    }

    let result = socket.next().await.unwrap().unwrap();
    Some(result)
}

async fn process_message(message: Message) -> Result<String, JobErrors> {
    match message {
        Message::Text(m) => { Ok(m) }
        Message::Close(_) => { Err(ConnectionClosed) }
        _ => { Err(TypeNotSupported) }
    }
}
