use config::get;

fn main() {
    // 获取 ollama 配置
    if let Some(ollama_config) = get("ollama") {
        println!("Ollama Config: {:?}", ollama_config);
    } else {
        println!("Ollama config not found.");
    }

    // 获取 hoopics 配置
    if let Some(hoopics_config) = get("hoopics") {
        println!("Hoopics Config: {:?}", hoopics_config);
    } else {
        println!("Hoopics config not found.");
    }
}