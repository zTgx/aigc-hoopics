use crate::middleware::auth::User;
use inspector::Inspector;
use primitives::job_req::{JobParams, JobResponse};
use std::convert::Infallible;
use warp::http::StatusCode;
use warp::reply::Reply;
use worker::add_job;

pub async fn handle_request(
    _user: User,
    job_params: JobParams,
) -> Result<impl Reply, warp::Rejection> {
    if let Err(e) = job_params.inspect() {
        let reject = warp::reject::custom(ValidationError(e));
        return Err(reject);
    }

    add_job(job_params.into()).await;

    let response = JobResponse {
        message: "Job submitted to Job Pool".to_string(),
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        StatusCode::OK,
    ))
}

#[derive(Debug)]
struct ValidationError(String);

impl warp::reject::Reject for ValidationError {}

// Add this function to handle the custom rejection
pub async fn handle_rejection(err: warp::Rejection) -> Result<impl Reply, Infallible> {
    if let Some(ValidationError(msg)) = err.find() {
        Ok(warp::reply::with_status(
            warp::reply::json(&msg),
            warp::http::StatusCode::BAD_REQUEST,
        ))
    } else {
        Ok(warp::reply::with_status(
            warp::reply::json(&"Internal server error"),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
