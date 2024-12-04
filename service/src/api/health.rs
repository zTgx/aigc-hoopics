use warp::reply::Reply;
use primitives::health::HealthCheckResponse;

pub fn handle_request() -> impl Reply {
    // Create a response object
    let response = HealthCheckResponse {
        status: "Health check OK".to_string(),
    };

    // Return the JSON response
    warp::reply::json(&response)
}
