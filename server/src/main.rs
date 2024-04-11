use std::net::SocketAddr;

use axum::{
    Router, routing::get,
};

mod socket;

const SERVER: &str = "0.0.0.0:3000";
#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/ws", get(socket::handler));

    let listener = tokio::net::TcpListener::bind(SERVER).await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}
