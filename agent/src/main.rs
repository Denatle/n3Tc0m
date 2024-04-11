use std::time::Duration;

use tokio::time::sleep;

use crate::run::run;

mod run;
mod socket;
mod errors;

#[tokio::main]
async fn main() -> Result<(), tokio_tungstenite::tungstenite::Error> {
    loop {
        let socket = match socket::spawn_client().await {
            Ok(socket) => { socket }
            Err(e) => {
                println!("Error: {:#?}", e);
                continue;
            }
        };
        match run(socket).await {
            Ok(_) => unreachable!("WTF"),
            Err(e) => {
                println!("Socket closed with error, reconnecting\nError: {:#?}", e);
                sleep(Duration::from_secs(1)).await;
                continue;
            }
        };
    }
}