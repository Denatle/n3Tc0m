use std::time::Duration;

use tokio::time::sleep;

use crate::employee::run;

mod employee;
mod socket;


#[tokio::main]
async fn main() -> Result<(), tokio_tungstenite::tungstenite::Error> {
    loop {
        let socket = match socket::spawn_client().await {
            Ok(socket) => { socket }
            #[cfg(debug_assertions)]
            Err(e) => {
                println!("Socket didnt open, reconnecting\nError:\n{:#?}", e);
                sleep(Duration::from_secs(1)).await;
                continue;
            }
            #[cfg(not(debug_assertions))]
            Err(_) => {
                sleep(Duration::from_secs(1)).await;
                continue;
            } 
        };
        match run(socket).await {
            #[cfg(debug_assertions)]
            Ok(e) => {
                println!("Something went very wrong:\n{:#?}", e)
            }
            #[cfg(debug_assertions)]
            Err(e) => {
                println!("Socket closed with error, reconnecting\nError:\n{:#?}", e);
                sleep(Duration::from_secs(1)).await;
                continue;
            }
            #[cfg(not(debug_assertions))]
            Ok(_) => {}
            #[cfg(not(debug_assertions))]
            Err(_) => {
                sleep(Duration::from_secs(1)).await;
                continue;
            }
        };
    }
}