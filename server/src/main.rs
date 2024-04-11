#[macro_use]
extern crate log;

use std::net::SocketAddr;
use axum::{Router, routing::get};
use axum::routing::post;

mod socket;
mod employer;
mod http;

const SERVER: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let app = Router::new()
        .route("/ws", get(socket::handler))
        .route("/client/cli/agents", get(http::agents))
        .route("/client/cli/command", post(http::command));

    let listener = tokio::net::TcpListener::bind(SERVER).await.unwrap();
    info!("Started server");
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
