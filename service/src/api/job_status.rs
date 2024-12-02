use serde::{Deserialize, Serialize};
use warp::reply::Reply;
use warp::http::StatusCode;
use crate::middleware::auth::User;

// Define a struct for the incoming job IDs
#[derive(Debug, Deserialize, Serialize)]
pub struct JobStatusRequest {
    pub job_ids: Vec<String>, // Expecting an array of strings
}

// Define a struct for the response
#[derive(Debug, Serialize)]
pub struct JobStatusResponse {
    pub message: String,
}

// Handle the request for job status
pub async fn handle_request(_user: User, request: JobStatusRequest) -> Result<impl Reply, warp::Rejection> {
    // Process the incoming JSON data (job_ids)
    let response_message = format!("Received job IDs: {:?}", request.job_ids);

    // Create a response object
    let response = JobStatusResponse {
        message: response_message,
    };

    // Return the JSON response with a 200 OK status
    Ok(warp::reply::with_status(warp::reply::json(&response), StatusCode::OK))
}