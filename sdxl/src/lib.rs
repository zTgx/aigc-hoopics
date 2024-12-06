use config::CONFIG;
use primitives::{
    job_status::{JobResult, JobStatusReq},
    sdxl::{Img2ImgRequest, SDXLJobRequest},
    Job, JobType,
};
use reqwest::Client;
use serde::Serialize;
use std::time::Duration;

#[derive(Clone)]
pub struct SDXLClient {
    client: Client,
}

impl SDXLClient {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn handle(&self, job: Job) {
        if job.params.job_type == JobType::Txt2Img {
            return self.handle_txt_2_img(job).await;
        }

        if job.params.job_type == JobType::Img2Img {
            return self.handle_img_2_img(job).await;
        }
    }
}

impl SDXLClient {
    pub async fn handle_txt_2_img(&self, job: Job) {
        let data: SDXLJobRequest = job.into();
        let url = format!("{}/txt2img", CONFIG.sdxl.normal);

        self.post(&data, &url).await
    }

    pub async fn handle_img_2_img(&self, job: Job) {
        let data: Img2ImgRequest = job.into();
        let url = format!("{}/img2img", CONFIG.sdxl.normal);

        self.post(&data, &url).await
    }
}

impl SDXLClient {
    async fn post<T>(&self, data: &T, url: &str)
    where
        T: Serialize + ?Sized,
    {
        // Send the POST request asynchronously
        let response = self
            .client
            .post(url)
            .header("Content-Type", "application/json")
            .json(&data) // Automatically serializes the struct to JSON
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
}

impl SDXLClient {
    pub async fn handle_job_status(
        &self,
        request: JobStatusReq,
    ) -> Result<Vec<JobResult>, reqwest::Error> {
        let link = format!("{}/get_task_status_batch", CONFIG.sdxl.normal);

        // Send the POST request asynchronously
        let response = self
            .client
            .post(&link)
            .header("Content-Type", "application/json")
            .json(&request) // Automatically serializes the struct to JSON
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

        response.json::<Vec<JobResult>>().await
    }
}
