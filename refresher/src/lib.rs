use lazy_static::lazy_static;
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
        // 启动第一个定时器，每隔5秒执行一次任务
        task::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(5));
            loop {
                interval.tick().await;
                println!("SDXL task: Checking status...");
            }
        });

        // 启动第二个定时器，每隔10秒执行一次任务
        task::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(10));
            loop {
                interval.tick().await;
                println!("FLUX task: Checking another status...");
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
