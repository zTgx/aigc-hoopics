use crate::error::ServiceError;
use crate::middleware::auth::User;
use inspector::Inspector;
use ollama::Llama;
use primitives::ollama::{PromptRequest, PromptResponse};

pub async fn handle_request(
    _user: User,
    request: PromptRequest,
) -> Result<impl warp::Reply, warp::Rejection> {
    request.inspect().unwrap();

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

// pub async fn handle_rejection(err: warp::Rejection) -> Result<impl Reply, std::convert::Infallible> {
//     if let Some(app_error) = err.find::<ServiceError>() {
//         return Ok(warp::reply::with_status(app_error.reason.clone(), warp::http::StatusCode::BAD_REQUEST));
//     }

//     // Handle other types of rejections (e.g., not found)
//     eprintln!("Unhandled rejection: {:?}", err);
//     Ok(warp::reply::with_status("INTERNAL_SERVER_ERROR".to_string(), warp::http::StatusCode::INTERNAL_SERVER_ERROR))
// }
