use std::time::Duration;

use tokio::time::sleep;

use crate::run::run;

mod run;
mod socket;

#[tokio::main]
async fn main() -> Result<(), tokio_tungstenite::tungstenite::Error> {
    loop {
        let socket = match socket::spawn_client().await {
            Ok(socket) => { socket }
            Err(e) => {
                #[cfg(debug_assertions)]
                println!("Socket didnt open, reconnecting\nError:\n{:#?}", e);
                sleep(Duration::from_secs(1)).await;
                continue;
            }
        };
        match run(socket).await {
            Ok(e) => {
                #[cfg(debug_assertions)]
                println!("Something went very wrong:\n{:#?}", e)
            },
            Err(e) => {
                #[cfg(debug_assertions)]
                println!("Socket closed with error, reconnecting\nError:\n{:#?}", e);
                sleep(Duration::from_secs(1)).await;
                continue;
            }
        };
    }
}