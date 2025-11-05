use crate::api::AppState;
use common::api;
use std::convert::TryInto;
use std::{sync::Arc, time::Duration};
use uuid::Uuid;
use warp::http::StatusCode;

const LONG_POLL_TIMEOUT_SECS: u64 = 5;
const POLL_INTERVAL_SECS: u64 = 1;

pub async fn create_job(
    state: Arc<AppState>,
    input: api::CreateJob,
) -> Result<impl warp::Reply, warp::Rejection> {
    let job: api::Job = state.service.create_job(input).await?.into();
    Ok(warp::reply::with_status(
        warp::reply::json(&api::Response::ok(job)),
        StatusCode::OK,
    ))
}

pub async fn post_job_result(
    state: Arc<AppState>,
    input: api::UpdateJobResult,
) -> Result<impl warp::Reply, warp::Rejection> {
    state.service.update_job_result(input).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&api::Response::ok(true)),
        StatusCode::OK,
    ))
}

pub async fn get_job_result(
    state: Arc<AppState>,
    job_id: Uuid,
) -> Result<impl warp::Reply, warp::Rejection> {
    const SLEEP_DURATION: Duration = Duration::from_secs(POLL_INTERVAL_SECS);

    // long polling: check immediately first, then poll
    for _ in 0..LONG_POLL_TIMEOUT_SECS {
        if let Some(job) = state.service.get_job_result(job_id).await? {
            return Ok(warp::reply::with_status(
                warp::reply::json(&api::Response::ok(job.into())),
                StatusCode::OK,
            ));
        }
        tokio::time::sleep(SLEEP_DURATION).await;
    }

    // if no job is found, return empty response
    Ok(warp::reply::with_status(
        warp::reply::json(&api::Response::<Option<()>>::ok(None)),
        StatusCode::OK,
    ))
}

pub async fn get_agent_job(
    state: Arc<AppState>,
    agent_id: Uuid,
) -> Result<impl warp::Reply, warp::Rejection> {
    const SLEEP_DURATION: Duration = Duration::from_secs(POLL_INTERVAL_SECS);

    // long polling: check immediately first, then poll
    for _ in 0..LONG_POLL_TIMEOUT_SECS {
        if let Some(job) = state.service.get_agent_job(agent_id).await? {
            let agent_job = api::AgentJob {
                id: job.id,
                encrypted_job: job.encrypted_job,
                ephemeral_public_key: job
                    .ephemeral_public_key
                    .try_into()
                    .expect("get_agent_job: invalid ephemeral_public_key"),
                nonce: job.nonce.try_into().expect("get_agent_job: invalid nonce"),
                signature: job.signature,
            };

            return Ok(warp::reply::with_status(
                warp::reply::json(&api::Response::ok(agent_job)),
                StatusCode::OK,
            ));
        }
        tokio::time::sleep(SLEEP_DURATION).await;
    }

    // if no job is found, return empty response
    Ok(warp::reply::with_status(
        warp::reply::json(&api::Response::<Option<()>>::ok(None)),
        StatusCode::OK,
    ))
}
