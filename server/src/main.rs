use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use common::api::{DataString};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/data", post(data));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn data(
    Json(payload): Json<DataString>,
) -> StatusCode {
    println!("{}", payload.string);
    StatusCode::ACCEPTED
}

// 
// #[derive(Serialize)]
// struct User {
//     id: u64,
//     username: String,
// }