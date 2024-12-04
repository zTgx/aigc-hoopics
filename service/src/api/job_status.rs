use crate::middleware::auth::User;
use inspector::Inspector;
use primitives::job_status::JobStatusReq;
use sdxl::handle_job_status;
use warp::http::StatusCode;
use warp::reply::Reply;

type JobStatusRequest = JobStatusReq;

// Handle the request for job status
pub async fn handle_request(
    _user: User,
    request: JobStatusRequest,
) -> Result<impl Reply, warp::Rejection> {
    request.inspect().unwrap();

    let status = handle_job_status(request.job_ids).await;

    // Return the JSON response with a 200 OK status
    Ok(warp::reply::with_status(
        warp::reply::json(&status),
        StatusCode::OK,
    ))
}
