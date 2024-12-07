use colored::Colorize;
use config::CONFIG;
use dispatcher::Dispatcher;
use lazy_static::lazy_static;
use primitives::ModelType;
use psql::engine::Engine;
use std::time::Duration;
use tokio::task;

// 定义Refresher结构体，用于封装需要定时执行的任务逻辑等相关功能
pub struct Refresher;

impl Refresher {
    // 创建一个新的Refresher实例的函数
    fn new() -> Self {
        Refresher {}
    }

    // 启动定时器任务的函数，这里启动两个不同间隔的定时器
    pub fn start(&self) {
        let dispatcher = Dispatcher::new();
        let should_print_log = CONFIG.refresher.print_log;
        let sdxl_interval = CONFIG.refresher.sdxl_interval;
        let flux_interval = CONFIG.refresher.flux_interval;

        // 启动第一个定时器，每隔5秒执行一次任务
        let dispatcher_clone = dispatcher.clone(); // 克隆 dispatcher，以便在异步任务中使用
        task::spawn(async move {
            println!(
                "{}",
                "✅ The Refresher service has started working\n".green()
            );

            let mut engine = Engine::new().await;

            let mut interval = tokio::time::interval(Duration::from_secs(sdxl_interval));
            loop {
                interval.tick().await;

                if should_print_log {
                    println!("{}", "SDXL task: Checking status ...".yellow());
                }

                if let Ok(ids) = engine
                    .fetch_pending_or_processing_job_ids(ModelType::SDXL)
                    .await
                {
                    if ids.len() == 0 {
                        continue;
                    }

                    match dispatcher_clone.check_status(ids.into()).await {
                        Ok(result) => {
                            if should_print_log {
                                println!("{}\n{:#?}", "SDXL task status: ".green(), result);

                                // # 状态更新
                                if let Err(e) = engine.update_job_status(&result).await {
                                    eprintln!("Error update job_status failed, reason: {}", e);
                                }

                                // # output_image_url 更新
                                if let Err(e) = engine.update_output_image_url(&result).await {
                                    eprintln!("Error update job_status failed, reason: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error checking SDXL status: {}", e);
                        }
                    }
                }
            }
        });

        // 启动第二个定时器，每隔10秒执行一次任务
        let dispatcher_clone = dispatcher.clone(); // 克隆 dispatcher，以便在异步任务中使用
        task::spawn(async move {
            let mut engine = Engine::new().await;
            let mut interval = tokio::time::interval(Duration::from_secs(flux_interval));

            loop {
                interval.tick().await;

                if should_print_log {
                    println!("{}", "FLUX task: Checking status ...".purple());
                }

                if let Ok(ids) = engine
                    .fetch_pending_or_processing_job_ids(ModelType::FLUX)
                    .await
                {
                    if ids.len() == 0 {
                        continue;
                    }

                    match dispatcher_clone.check_status(ids.into()).await {
                        Ok(result) => {
                            if should_print_log {
                                println!("{}\n{:#?}", "FLUX task status: ".purple(), result);

                                // # 状态更新
                                if let Err(e) = engine.update_job_status(&result).await {
                                    eprintln!("Error update job_status failed, reason: {}", e);
                                }

                                // # output_image_url 更新
                                if let Err(e) = engine.update_output_image_url(&result).await {
                                    eprintln!("Error update job_status failed, reason: {}", e);
                                }
                            }
                        }
                        Err(e) => {
                            eprintln!("Error checking SDXL status: {}", e);
                        }
                    }
                }
            }
        });
    }
}

// 使用lazy_static定义全局的Refresher实例，延迟初始化
lazy_static! {
    pub static ref REFRESHER: Refresher = {
        let r = Refresher::new();
        r
    };
}
