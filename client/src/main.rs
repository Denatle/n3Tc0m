use common::api;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = reqwest::Client::new();
    let body = api::DataString {
        string: "".into()
    };
    client.post("0.0.0.0:3000").body(body).send().await?;

    Ok(())
}