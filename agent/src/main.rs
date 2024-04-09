mod run;
mod network;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // run::run().await?;
    network::spawn_client(1).await;
    Ok(())
}