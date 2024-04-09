use axum::{
    routing::{get, post},
    http::StatusCode,
    Json, Router,
};
use common::api::{CommandResult, Job};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/data", post(data))
        .route("/jobs", get(jobs));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn data(
    Json(payload): Json<CommandResult>,
) -> StatusCode {
    println!("{}", payload.output);
    StatusCode::ACCEPTED
}

async fn jobs() -> Json<Job> {
    let new_job: Job = Job {
        command: "ls".into(),
        args: vec!["..".into()],
    };
    Json(new_job)
}