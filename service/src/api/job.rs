use crate::middleware::auth::User;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::Reply;
use worker::{submit_job, Job};

// Define a struct for the incoming job IDs
#[derive(Debug, Deserialize, Serialize)]
pub struct JobRequest {
    pub job: String,
}

// Define a struct for the response
#[derive(Debug, Serialize)]
pub struct JobResponse {
    pub message: String,
}

// Handle the request for job status
pub async fn handle_request(
    _user: User,
    request: JobRequest,
) -> Result<impl Reply, warp::Rejection> {
    // Process the incoming JSON data (job_ids)
    let job_id = Uuid::new_v4();
    let job = Job {
        id: job_id.to_string(),
        params: json!(true),
    };

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
