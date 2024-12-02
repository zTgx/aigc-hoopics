use ollama::Llama;
use serde::{Deserialize, Serialize};
use crate::api::error::AppError;
use crate::middleware::auth::User;

// Define your structs
#[derive(Debug, Deserialize, Serialize)]
pub struct PromptRequest {
    pub prompt: String, // 用户输入的prompt
}

#[derive(Debug, Deserialize, Serialize)]
pub struct PromptResponse {
    pub original_prompt: String, // 用户自己写的prompt
    pub updated_prompt: String,  // 经过ollama处理后的prompt
}

pub async fn handle_request(_user: User, request: PromptRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let ollama = Llama::new();

    // Attempt to get the updated prompt and handle potential errors
    let updated_prompt = ollama.prompt(&request.prompt.clone()).await.map_err(|e| {
        // Map the error to a custom rejection
        warp::reject::custom(AppError{ reason: format!("Failed to process prompt: {}", e) })
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
//     if let Some(app_error) = err.find::<AppError>() {
//         return Ok(warp::reply::with_status(app_error.reason.clone(), warp::http::StatusCode::BAD_REQUEST));
//     }

//     // Handle other types of rejections (e.g., not found)
//     eprintln!("Unhandled rejection: {:?}", err);
//     Ok(warp::reply::with_status("INTERNAL_SERVER_ERROR".to_string(), warp::http::StatusCode::INTERNAL_SERVER_ERROR))
// }