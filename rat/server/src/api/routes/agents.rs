use crate::api::AppState;
use common::api;
use std::sync::Arc;
use uuid::Uuid;
use warp::{http::StatusCode, Rejection};

pub async fn get_agents(state: Arc<AppState>) -> Result<impl warp::Reply, Rejection> {
    let agents: Vec<api::Agent> = state
        .service
        .list_agents()
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(warp::reply::with_status(
        warp::reply::json(&api::Response::ok(api::AgentsList { agents })),
        StatusCode::OK,
    ))
}

pub async fn get_agent(
    state: Arc<AppState>,
    agent_id: Uuid,
) -> Result<impl warp::Reply, Rejection> {
    let agent: api::Agent = state.service.find_agent(agent_id).await?.into();
    Ok(warp::reply::with_status(
        warp::reply::json(&api::Response::ok(agent)),
        StatusCode::OK,
    ))
}

pub async fn post_agents(
    state: Arc<AppState>,
    input: api::RegisterAgent,
) -> Result<impl warp::Reply, Rejection> {
    let agent_info = state.service.register_agent(input).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&api::Response::ok(agent_info)),
        StatusCode::OK,
    ))
}
