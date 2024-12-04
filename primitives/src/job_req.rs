
use inspector::Inspector;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::Job;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JobParams {
    pub prompt: String,
    pub negative_prompt: String,
    pub job_type: i32, // Assuming job_type is an integer
    pub img_link: String,
    pub priority: i32, // Assuming priority is an integer
    pub description: String,
    pub job_style: String,
    pub model: String,
    pub width: u32,  // Assuming width is an unsigned integer
    pub height: u32, // Assuming height is an unsigned integer
    pub rewrite_prompt: bool,
}

impl From<JobParams> for Job {
    fn from(params: JobParams) -> Self {
        Job {
            id: Uuid::new_v4().to_string(),
            params
        }
    }
}

impl Inspector for JobParams {
    fn inspect(&self) -> Result<bool, String> {
        Ok(true)
    }
}

#[derive(Debug, Serialize)]
pub struct JobResponse {
    pub message: String,
}