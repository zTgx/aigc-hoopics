use colored::Colorize;
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

        // 启动第一个定时器，每隔5秒执行一次任务
        let dispatcher_clone = dispatcher.clone(); // 克隆 dispatcher，以便在异步任务中使用
                                                   // 启动第一个定时器，每隔5秒执行一次任务
        task::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(5));
            loop {
                interval.tick().await;
                println!("{}", "SDXL task: Checking status...".yellow());

                let ids = Engine::fetch_pending_or_processing_job_ids(ModelType::SDXL);
                match dispatcher_clone.check_status(ids.into()).await {
                    Ok(result) => {
                        println!("{}\n{:#?}", "SDXL job status: ".yellow(), result);
                    }
                    Err(e) => {
                        eprintln!("Error checking SDXL status: {}", e);
                    }
                }
            }
        });

        // 启动第二个定时器，每隔10秒执行一次任务
        let dispatcher_clone = dispatcher.clone(); // 克隆 dispatcher，以便在异步任务中使用
                                                   // 启动第二个定时器，每隔10秒执行一次任务
        task::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                println!("{}", "FLUX task: Checking another status...".purple());

                let ids = Engine::fetch_pending_or_processing_job_ids(ModelType::FLUX);
                match dispatcher_clone.check_status(ids.into()).await {
                    Ok(result) => {
                        println!("{}\n{:#?}", "FLUX job status: ".purple(), result);
                    }
                    Err(e) => {
                        eprintln!("Error checking FLUX status: {}", e);
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
