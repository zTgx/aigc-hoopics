#[tokio::main]
async fn main() {
    refresher::REFRESHER.start();
    service::start().await;
}
