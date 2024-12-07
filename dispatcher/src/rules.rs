use std::collections::HashMap;

use primitives::job_status::JobStatusReq;
use psql::engine::get_global_engine;
use tokio_postgres::Error;

// Key: SDXL / FLUX
// Value: job_ids
type SDXLRules = HashMap<String, Vec<String>>;

// 分发Job的规则
pub async fn check_status_rules(request: &JobStatusReq) -> Result<SDXLRules, Error> {
    let engine = get_global_engine().await;
    let mut engine_lock = engine.lock().await;

    engine_lock.sort_jobs_by_model(&request.job_ids).await
}
