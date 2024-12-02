use serde::{Deserialize, Serialize};
use warp::reply::Json;

use crate::middleware::auth::User;

// Define a struct for the incoming job IDs
#[derive(Debug, Deserialize, Serialize)]
pub struct JobStatusRequest {
    pub job_ids: Vec<String>, // Expecting an array of strings
}

pub fn execute(_user: User, request: JobStatusRequest) -> Json {
    // Process the incoming JSON data (job_ids)
    let response_message = format!("Received job IDs: {:?}", request.job_ids);
    warp::reply::json(&response_message)
    
}