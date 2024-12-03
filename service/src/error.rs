use warp::reject;

// Define a custom error type if needed
#[derive(Debug)]
pub struct ServiceError {
    pub reason: String,
}

impl reject::Reject for ServiceError {}
