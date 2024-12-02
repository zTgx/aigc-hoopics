use serde::Serialize;
use warp::reply::Reply;

// Define a struct for the health check response
#[derive(Serialize)]
struct HealthCheckResponse {
    status: String,
}

pub fn handle_request() -> impl Reply {
    // Create a response object
    let response = HealthCheckResponse {
        status: "Health check OK".to_string(),
    };

    // Return the JSON response
    warp::reply::json(&response)
}