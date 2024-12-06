use crate::error::ServiceError;
use crate::middleware::auth::User;
use inspector::Inspector;
use ollama::Llama;
use primitives::ollama::{PromptRequest, PromptResponse};
use std::convert::Infallible;

pub async fn handle_request(
    _user: User,
    request: PromptRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = request.inspect() {
        let reject = warp::reject::custom(ValidationError(e));
        return Err(reject);
    }

    let ollama = Llama::new();

    // Attempt to get the updated prompt and handle potential errors
    let updated_prompt = ollama.prompt(&request.prompt.clone()).await.map_err(|e| {
        // Map the error to a custom rejection
        warp::reject::custom(ServiceError {
            reason: format!("Failed to process prompt: {}", e),
        })
    })?;

    // Create the response object
    let prompt_response = PromptResponse {
        original_prompt: request.prompt,
        updated_prompt,
    };

    // Return the JSON response
    Ok(warp::reply::json(&prompt_response))
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
