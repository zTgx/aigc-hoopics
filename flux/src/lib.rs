use colored::Colorize;
use config::CONFIG;
use primitives::{
    job_status::{JobResult, JobStatusReq},
    sdxl::{Img2ImgRequest, Txt2ImgRequest},
    select_flux_config, Job, JobType,
};
use reqwest::{Client, Error, Response};
use serde::Serialize;
use std::time::Duration;

#[derive(Clone)]
pub struct FLUXClient {
    client: Client,
}

impl FLUXClient {
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

impl FLUXClient {
    async fn handle_txt_2_img(&self, job: Job) -> Result<Response, reqwest::Error> {
        let url = format!("{}/txt2img", select_flux_config!(&job.job_style));

        let data = Txt2ImgRequest::new(job);
        self.post(&data, &url).await
    }

    async fn handle_img_2_img(&self, job: Job) -> Result<Response, reqwest::Error> {
        let url = format!("{}/img2img", select_flux_config!(&job.job_style));

        let data = Img2ImgRequest::new(job);
        self.post(&data, &url).await
    }
}

impl FLUXClient {
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
            println!("{}", "FLUX Job request sent successfully!".green());
        } else {
            eprintln!("Failed to send job request: {}", response.status());
        }

        Ok(response)
    }
}

impl FLUXClient {
    pub async fn handle_job_status(
        &self,
        request: JobStatusReq,
    ) -> Result<Vec<JobResult>, reqwest::Error> {
        let link = format!("{}/get_task_status_batch", CONFIG.flux.normal);

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
            println!("{}", "FLUX Job status request sent successfully!".green());
        } else {
            eprintln!("Failed to send job request: {}", response.status());
        }

        response.json::<Vec<JobResult>>().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Result;

    #[test]
    fn test_parse_job_results() -> Result<()> {
        let json_data = r#"[{"file_urls":["https://nft1000.oss-cn-beijing.aliyuncs.com/sd_output/txt2img/2024-12-07/t2i-2024-12-07-14-07-18-a123a974-a6a3-4e0f-883a-41fcaf41a93c-0.png"],"job_id":"a123a974-a6a3-4e0f-883a-41fcaf41a93c","status":"success"},{"file_urls":["https://nft1000.oss-cn-beijing.aliyuncs.com/sd_output/txt2img/2024-12-07/t2i-2024-12-07-19-53-13-0e738834-21b8-4d11-85b4-7253768b0e76-0.png"],"job_id":"0e738834-21b8-4d11-85b4-7253768b0e76","status":"success"}]"#;

        let results: Vec<JobResult> = serde_json::from_str(json_data)?;

        // Expected results
        let expected = vec![
            JobResult {
                file_urls: vec![
                    "https://nft1000.oss-cn-beijing.aliyuncs.com/sd_output/txt2img/2024-12-07/t2i-2024-12-07-14-07-18-a123a974-a6a3-4e0f-883a-41fcaf41a93c-0.png".to_string(),
                ],
                job_id: "a123a974-a6a3-4e0f-883a-41fcaf41a93c".to_string(),
                status: "success".to_string(),
            },
            JobResult {
                file_urls: vec![
                    "https://nft1000.oss-cn-beijing.aliyuncs.com/sd_output/txt2img/2024-12-07/t2i-2024-12-07-19-53-13-0e738834-21b8-4d11-85b4-7253768b0e76-0.png".to_string(),
                ],
                job_id: "0e738834-21b8-4d11-85b4-7253768b0e76".to_string(),
                status: "success".to_string(),
            },
        ];

        // Assert that parsed results match expected values
        assert_eq!(results, expected);

        Ok(())
    }
}
