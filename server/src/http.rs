use axum::http::StatusCode;
use axum::Json;

use axum_auth::AuthBasic;
use common::api::{Agents, CommandOutput, CommandResult, TargetJob};

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

pub(crate) async fn command(AuthBasic((id, password)): AuthBasic, Json(target_job): Json<TargetJob>) -> (StatusCode, Json<CommandResult>) {
    let password = match password {
        Some(password) => { password }
        None => { return (StatusCode::NOT_FOUND, Json(CommandResult::CommandOutput(CommandOutput::default()))); }
    };
    info!("Password = {password}, id {id}");
    let result = execute_job(target_job.socket_addr, target_job.job).await;
    (StatusCode::OK, Json(result))
}