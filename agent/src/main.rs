mod run;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    run::run().await?;
    Ok(())
}