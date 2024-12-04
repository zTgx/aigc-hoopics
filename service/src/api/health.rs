use primitives::health::HealthCheckResponse;
use warp::reply::Reply;

pub fn handle_request() -> impl Reply {
    // Create a response object
    let response = HealthCheckResponse {
        status: "Health check OK".to_string(),
    };

    // Return the JSON response
    warp::reply::json(&response)
}
