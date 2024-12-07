use config::CONFIG;

fn main() {
    // 获取 ollama 配置
    let ollama_config = &CONFIG.ollama;
    println!("Ollama Config: {:#?}", ollama_config);
}
