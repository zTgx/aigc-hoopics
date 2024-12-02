use warp::reject;

// Define a custom error type if needed
#[derive(Debug)]
pub struct AppError {
    pub reason: String,
}

impl reject::Reject for AppError {}