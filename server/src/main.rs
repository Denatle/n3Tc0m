#[macro_use] extern crate log;
use std::net::SocketAddr;

use axum::{
    Router, routing::get,
};

mod socket;
mod employer;
mod errors;
const SERVER: &str = "0.0.0.0:3000";

#[tokio::main]
async fn main() {
    env_logger::init();
    let app = Router::new()
        .route("/ws", get(socket::handler));

    let listener = tokio::net::TcpListener::bind(SERVER).await.unwrap();
    info!("Started server");
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
