pub mod rules;

use primitives::Job;
use sdxl::handle_sdxl_service;

pub async fn dispatch(job: Job) {
    handle_sdxl_service(job).await;
}