use axum::http::StatusCode;
use axum::Json;

use axum_auth::AuthBasic;
use common::api::{Agents, JobOutput, JobResult, TargetJob};
use common::errors::JobErrors;

use crate::employer::{execute_job, get_agents};

pub(crate) async fn agents(AuthBasic((id, password)): AuthBasic) -> (StatusCode, Json<Agents>) {
    let password = match password {
        Some(password) => { password }
        None => { return (StatusCode::NOT_FOUND, Json(Agents::default())); }
    };
    info!("Password = {password}, id {id}");
    (StatusCode::OK, Json(Agents {
        socket_addrs: get_agents().await
    }))
}

pub(crate) async fn command(AuthBasic((id, password)): AuthBasic, Json(target_job): Json<TargetJob>) -> (StatusCode, Json<JobResult>) {
    let password = match password {
        Some(password) => { password }
        None => { return (StatusCode::NOT_FOUND, Json(JobResult { id: 0, job_output: Ok(JobOutput::default()) })); }
    };
    info!("Password = {password}, id {id}");

    let result = execute_job(target_job.socket_addr, target_job).await;
    match result.job_output {
        Ok(_) => { (StatusCode::OK, Json(result)) }
        Err(ref e) => {
            match e {
                JobErrors::SocketNotFound => { (StatusCode::NOT_FOUND, Json(result)) }
                JobErrors::ConnectionClosed => { (StatusCode::INTERNAL_SERVER_ERROR, Json(result)) }
                JobErrors::TypeNotSupported => { (StatusCode::BAD_REQUEST, Json(result)) }
                JobErrors::NoMessage => { (StatusCode::INTERNAL_SERVER_ERROR, Json(result)) }
                JobErrors::BadAgentMessage => { (StatusCode::FAILED_DEPENDENCY, Json(result)) }
                JobErrors::TimeOut => { (StatusCode::REQUEST_TIMEOUT, Json(result)) }
            }
        }
    }
}