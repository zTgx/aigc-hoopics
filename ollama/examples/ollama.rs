use ollama::Llama;

#[tokio::main]
async fn main() {
    // 创建 Llama 实例并调用其方法
    let llama = Llama::new();
    let response = llama.prompt("在他乡").await.unwrap();
    println!("Response from Llama: {}", response);
}