use colored::Colorize;
use config::CONFIG;
use primitives::{
    job_status::{JobResult, JobStatusReq},
    sdxl::{Img2ImgRequest, SDXLJobRequest},
    Job, JobType,
};
use reqwest::{Client, Error, Response};
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

    pub async fn handle(&self, job: Job) -> Result<Response, reqwest::Error> {
        match job.job_type {
            JobType::Txt2Img => {
                return self.handle_txt_2_img(job).await;
            }
            JobType::Img2Img => {
                return self.handle_img_2_img(job).await;
            }
        }
    }
}

impl SDXLClient {
    pub async fn handle_txt_2_img(&self, job: Job) -> Result<Response, reqwest::Error> {
        let data = SDXLJobRequest::new(job);
        println!("SDXL SDXLJobRequest: {:#?}", data);

        let url = format!("{}/txt2img", CONFIG.sdxl.normal);

        self.post(&data, &url).await
    }

    pub async fn handle_img_2_img(&self, job: Job) -> Result<Response, reqwest::Error> {
        let data = Img2ImgRequest::new(job);
        let url = format!("{}/img2img", CONFIG.sdxl.normal);

        self.post(&data, &url).await
    }
}

impl SDXLClient {
    async fn post<T>(&self, data: &T, url: &str) -> Result<Response, Error>
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
            .await?;

        // Check if the response was successful
        if response.status().is_success() {
            println!("{}", "SDXL Job request sent successfully!".green());
        } else {
            eprintln!("Failed to send job request: {}", response.status());
        }

        Ok(response)
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
            .await?;

        // Check if the response was successful
        if response.status().is_success() {
            // println!("Job request sent successfully!");
        } else {
            eprintln!("Failed to send job request: {}", response.status());
        }

        response.json::<Vec<JobResult>>().await
    }
}
