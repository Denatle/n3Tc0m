#[macro_use]
extern crate log;

use std::net::{SocketAddr};

use axum::{Router, routing::get};
use axum::routing::post;


mod socket;
mod employer;
mod http;

const SERVER: &str = "localhost:45000";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/ws", get(socket::handler))
        .route("/client/cli/agents", get(http::agents))
        .route("/client/cli/command", post(http::command));

    // println!("{:#?}", serde_json::to_string(&TargetJob {
    //     socket_addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 20000),
    //     job_type: JobType::Command(CommandPayload {
    //         command: "echo".to_string(),
    //         args: vec!["hello".to_string(), "world".to_string()],
    //     }),
    //     timeout: Default::default(),
    // }).unwrap());

    let listener = tokio::net::TcpListener::bind(SERVER).await.unwrap();
    info!("Started server on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
