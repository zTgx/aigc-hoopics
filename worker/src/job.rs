use inspector::Inspector;
use sdxl::SDXLJobRequest;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Job {
    pub id: String,
    pub params: JobParams,
}

impl Job {
    pub fn new(id: String, params: JobParams) -> Self {
        Self { id, params }
    }
}

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

// Define a struct for the incoming job IDs
#[derive(Debug, Deserialize, Serialize)]
pub struct JobStatusReq {
    pub job_ids: Vec<String>, // Expecting an array of strings
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobResult {
    pub file_urls: Vec<String>, // A vector to hold multiple file URLs
    pub job_id: String,         // The job ID
    pub status: String,         // The status of the job
}

impl Inspector for JobParams {
    fn inspect(&self) -> bool {
        true
    }
}

impl From<Job> for SDXLJobRequest {
    fn from(item: Job) -> Self {
        SDXLJobRequest {
            prompt: item.params.prompt,
            job_id: item.id,
            style: item.params.job_style,
            model_type: item.params.model,
            width: item.params.width,
            height: item.params.height,
        }
    }
}
