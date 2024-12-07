use inspector::Inspector;
use serde::{Deserialize, Serialize};

// Define a struct for the incoming job IDs
#[derive(Debug, Deserialize, Serialize)]
pub struct JobStatusReq {
    pub job_ids: Vec<String>, // Expecting an array of strings
}

impl From<Vec<String>> for JobStatusReq {
    fn from(job_ids: Vec<String>) -> Self {
        JobStatusReq { job_ids }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct JobResult {
    pub file_urls: Vec<String>, // A vector to hold multiple file URLs
    pub job_id: String,         // The job ID
    pub status: String,         // The status of the job
}

impl Inspector for JobStatusReq {
    fn inspect(&self) -> Result<bool, String> {
        Ok(true)
    }
}
