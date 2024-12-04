use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SDXLJobRequest {
    pub prompt: String,
    pub job_id: String,
    pub style: String,
    pub model_type: String,
    pub width: u32,
    pub height: u32,
}
