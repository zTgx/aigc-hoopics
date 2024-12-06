use crate::middleware::auth::User;
use inspector::Inspector;
use ollama::Llama;
use primitives::job_req::{JobParams, JobResponse};
use primitives::Job;
use psql::engine::Engine;
use std::convert::Infallible;
use uuid::Uuid;
use warp::http::StatusCode;
use warp::reply::Reply;
use worker::add_job;

pub async fn handle_request(_user: User, param: JobParams) -> Result<impl Reply, warp::Rejection> {
    if let Err(e) = param.inspect() {
        let reject = warp::reject::custom(ValidationError(e));
        return Err(reject);
    }

    let job = re_mapping_job(param).await;
    Engine::new().save_job(&job);

    add_job(job).await;

    let response = JobResponse {
        message: "Job submitted to Job Pool".to_string(),
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        StatusCode::OK,
    ))
}

async fn re_mapping_job(param: JobParams) -> Job {
    let job_id = Uuid::new_v4().to_string();

    let updated_prompt = {
        if param.rewrite_prompt {
            let ollama = Llama::new();
            let prompt = ollama.prompt(&param.prompt).await.unwrap();

            prompt
        } else {
            param.prompt
        }
    };

    Job {
        id: job_id,
        prompt: updated_prompt,
        negative_prompt: param.negative_prompt,
        job_type: param.job_type,
        img_link: param.img_link,
        priority: param.priority,
        description: param.description,
        job_style: param.job_style,
        model: param.model,
        width: param.width,
        height: param.height,
        rewrite_prompt: param.rewrite_prompt,
    }
}

#[derive(Debug)]
struct ValidationError(String);

impl warp::reject::Reject for ValidationError {}

// Add this function to handle the custom rejection
pub async fn handle_rejection(err: warp::Rejection) -> Result<impl Reply, Infallible> {
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
