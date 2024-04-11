use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub enum CommandErrors {
    ConnectionClosed,
    TypeNotSupported,
    NoMessage,
    BadMessage
}
