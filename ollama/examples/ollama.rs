use ollama::Llama;

#[tokio::main]
async fn main() {
    // 创建 Llama 实例并调用其方法
    let llama = Llama::new();
    let response = llama
        .prompt("在他乡")
        .await
        .expect("Ollama service goes wild.");
    println!("Response from Llama: {}", response);
}
