use crate::middleware::auth::User;
use serde::Serialize;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::Reply;
use worker::job::{Job, JobParams};
use worker::submit_job;
use inspector::Inspector;

pub type JobRequest = JobParams;

#[derive(Debug, Serialize)]
pub struct JobResponse {
    pub message: String,
}

pub async fn handle_request(
    _user: User,
    job_params: JobRequest,
) -> Result<impl Reply, warp::Rejection> {
    job_params.inspect();
    let job = Job::new(Uuid::new_v4().to_string(), job_params);

    println!("Receive job: {:?}", job);

    // 直接发给Worker立马返回
    submit_job(job);

    // Create a response object
    let response = JobResponse {
        message: "Job submitted to Job Pool".to_string(),
    };

    // Return the JSON response with a 200 OK status
    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        StatusCode::OK,
    ))
}
