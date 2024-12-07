use std::convert::Infallible;

use crate::middleware::auth::User;
use inspector::Inspector;
use primitives::job_status::JobStatusReq;
use warp::http::StatusCode;
use warp::reply::Reply;

type JobStatusRequest = JobStatusReq;

// Handle the request for job status
pub async fn handle_request(
    _user: User,
    request: JobStatusRequest,
) -> Result<impl Reply, warp::Rejection> {
    if let Err(e) = request.inspect() {
        let reject = warp::reject::custom(ValidationError(e));
        return Err(reject);
    }

    match dispatcher::Dispatcher::new().check_status(request).await {
        Ok(result) => {
            // Return the JSON response with a 200 OK status
            Ok(warp::reply::with_status(
                warp::reply::json(&result),
                StatusCode::OK,
            ))
        }
        Err(e) => {
            eprintln!("Error checking FLUX status: {}", e);

            let reject = warp::reject::custom(ValidationError(e.to_string()));
            Err(reject)
        }
    }
}
#[derive(Debug)]
struct ValidationError(String);
impl warp::reject::Reject for ValidationError {}

// Add this function to handle the custom rejection
pub async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, Infallible> {
    if let Some(ValidationError(msg)) = err.find() {
        Ok(warp::reply::with_status(
            warp::reply::json(&msg),
            warp::http::StatusCode::BAD_REQUEST,
        ))
    } else {
        Ok(warp::reply::with_status(
            warp::reply::json(&"Internal server error"),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}
