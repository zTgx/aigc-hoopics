use serde::Serialize;

// Define a struct for the health check response
#[derive(Serialize)]
pub struct HealthCheckResponse {
    pub status: String,
}