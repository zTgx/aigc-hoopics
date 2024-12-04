use crate::middleware::auth::User;
use inspector::Inspector;
use primitives::job_req::{JobParams, JobResponse};
use warp::http::StatusCode;
use warp::reply::Reply;
use worker::add_job;

pub async fn handle_request(
    _user: User,
    job_params: JobParams,
) -> Result<impl Reply, warp::Rejection> {
    job_params.inspect().unwrap();

    add_job(job_params.into()).await;

    let response = JobResponse {
        message: "Job submitted to Job Pool".to_string(),
    };

    // Return the JSON response with a 200 OK status
    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        StatusCode::OK,
    ))
}
