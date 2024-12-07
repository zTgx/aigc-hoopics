pub mod api;
pub mod error;
pub mod middleware;
pub mod routes;

use colored::Colorize;
use config::CONFIG;
use routes::api_routes;
use std::net::SocketAddr;

pub async fn start() {
    print_logo();

    let addr = format!("{}:{}", CONFIG.service.endpoint, CONFIG.service.port);
    println!(
        "{} {}",
        "✅ The API service is running on → ".green(),
        addr.green()
    );

    let socket_addr: SocketAddr = addr.parse().expect("Invalid address format");
    warp::serve(api_routes()).run(socket_addr).await;
}

fn print_logo() {
    let logo = r#"
      ___           ___           ___           ___                       ___           ___     
     /\__\         /\  \         /\  \         /\  \          ___        /\  \         /\  \    
    /:/  /        /::\  \       /::\  \       /::\  \        /\  \      /::\  \       /::\  \   
   /:/__/        /:/\:\  \     /:/\:\  \     /:/\:\  \       \:\  \    /:/\:\  \     /:/\ \  \  
  /::\  \ ___   /:/  \:\  \   /:/  \:\  \   /::\~\:\  \      /::\__\  /:/  \:\  \   _\:\~\ \  \ 
 /:/\:\  /\__\ /:/__/ \:\__\ /:/__/ \:\__\ /:/\:\ \:\__\  __/:/\/__/ /:/__/ \:\__\ /\ \:\ \ \__\
 \/__\:\/:/  / \:\  \ /:/  / \:\  \ /:/  / \/__\:\/:/  / /\/:/  /    \:\  \  \/__/ \:\ \:\ \/__/
      \::/  /   \:\  /:/  /   \:\  /:/  /       \::/  /  \::/__/      \:\  \        \:\ \:\__\  
      /:/  /     \:\/:/  /     \:\/:/  /         \/__/    \:\__\       \:\  \        \:\/:/  /  
     /:/  /       \::/  /       \::/  /                    \/__/        \:\__\        \::/  /   
     \/__/         \/__/         \/__/                                   \/__/         \/__/    

"#;
    println!("{}", logo.yellow());
}
