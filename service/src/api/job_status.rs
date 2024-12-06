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
    request.inspect().unwrap();

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

            let reject = warp::reject::custom(Nope {});
            Err(reject)
        }
    }
}

#[derive(Debug)]
struct Nope {}
impl warp::reject::Reject for Nope {}
