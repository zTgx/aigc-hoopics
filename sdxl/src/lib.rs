use serde::{Deserialize, Serialize};
use config::CONFIG;
use reqwest::Client;

#[derive(Debug, Serialize, Deserialize)]
pub struct SDXLJobRequest {
    pub prompt: String,
    pub job_id: String,
    pub style: String,
    pub model_type: String,
    pub width: u32,
    pub height: u32,
}

pub async fn handle_sdxl_service(sdxl_request: SDXLJobRequest) {
    // Assuming CONFIG.sdxl.normal is a valid URL
    let link = format!("{}/txt2img", CONFIG.sdxl.normal);

    // Create a new HTTP client
    let client = Client::new();

    // Send the POST request
    let response = client.post(&link)
        .header("Content-Type", "application/json")
        .json(&sdxl_request) // Automatically serializes the struct to JSON
        .timeout(Duration::from_secs(6)) // Set timeout
        .send() // This is now a blocking call
        .unwrap(); // Handle errors appropriately in production code

    println!("sdxl response: {:?}", response);

    // Check if the response was successful
    if response.status().is_success() {
        println!("Job request sent successfully!");
    } else {
        eprintln!("Failed to send job request: {}", response.status());
    }
} 