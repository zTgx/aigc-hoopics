use warp::Filter;

use crate::{
    api::{health, submit_job, job_status, prompt},
    middleware::auth::with_auth,
};

pub fn api_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    v1_routes().or(v2_routes())
}

fn v1_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let health = warp::path!("v1" / "health").map(|| health::handle_request());

    let status = warp::path!("v1" / "job-status")
        .and(warp::post())
        .and(with_auth()) // Requires authorization
        .and(warp::body::json()) // Expecting JSON body
        .and_then(job_status::handle_request);

    let prompt = warp::path!("v1" / "prompt")
        .and(warp::post())
        .and(with_auth()) // Requires authorization
        .and(warp::body::json()) // Expecting JSON body
        .and_then(prompt::handle_request);

    let job = warp::path!("v1" / "submit-job")
        .and(warp::post())
        .and(with_auth()) // Requires authorization
        .and(warp::body::json()) // Expecting JSON body
        .and_then(submit_job::handle_request);

    // Combine routes
    health.or(status).or(prompt).or(job)
}

fn v2_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Define GET /v2/hello
    let hello_v2 = warp::path!("v2" / "hello").map(|| warp::reply::json(&"Hello from v2!"));

    // Define POST /v2/data
    let data_v2 = warp::path!("v2" / "data")
        .and(warp::post())
        .and(warp::body::json())
        .map(|body: serde_json::Value| {
            // Process the incoming JSON data
            warp::reply::json(&body)
        });

    // Combine routes
    hello_v2.or(data_v2)
}
