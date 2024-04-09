mod network;
mod ui;

use std::thread;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    thread::spawn(|| { ui::render().unwrap() }).join().unwrap();

    Ok(())
}
