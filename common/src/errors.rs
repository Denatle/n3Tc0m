use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum JobErrors {
    SocketNotFound,
    ConnectionClosed,
    TypeNotSupported,
    NoMessage,
    BadAgentMessage,
    TimeOut
}
