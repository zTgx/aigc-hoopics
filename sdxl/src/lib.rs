use config::CONFIG;
use primitives::sdxl::SDXLJobRequest;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub async fn handle_sdxl_service(sdxl_request: SDXLJobRequest) {
    // Assuming CONFIG.sdxl.normal is a valid URL
    let link = format!("{}/txt2img", CONFIG.sdxl.normal);

    // Create a new HTTP client
    let client = Client::new();

    // Send the POST request asynchronously
    let response = client
        .post(&link)
        .header("Content-Type", "application/json")
        .json(&sdxl_request) // Automatically serializes the struct to JSON
        .timeout(Duration::from_secs(6)) // Set timeout
        .send() // This is now an asynchronous call
        .await // Await the response
        .unwrap(); // Handle errors appropriately in production code

    println!("sdxl response: {:?}", response);

    // Check if the response was successful
    if response.status().is_success() {
        println!("Job request sent successfully!");
    } else {
        eprintln!("Failed to send job request: {}", response.status());
    }
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

pub async fn handle_job_status(job_ids: Vec<String>) -> Vec<JobResult> {
    // Assuming CONFIG.sdxl.normal is a valid URL
    let link = format!("{}/get_task_status_batch", CONFIG.sdxl.normal);

    let data = JobStatusReq { job_ids };

    // Create a new HTTP client
    let client = Client::new();

    // Send the POST request asynchronously
    let response = client
        .post(&link)
        .header("Content-Type", "application/json")
        .json(&data) // Automatically serializes the struct to JSON
        .timeout(Duration::from_secs(6)) // Set timeout
        .send() // This is now an asynchronous call
        .await // Await the response
        .unwrap(); // Handle errors appropriately in production code

    // Check if the response was successful
    if response.status().is_success() {
        println!("Job request sent successfully!");
    } else {
        eprintln!("Failed to send job request: {}", response.status());
    }

    let res = response.json::<Vec<JobResult>>().await;

    res.unwrap()
}
