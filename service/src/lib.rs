pub mod api;
pub mod error;
pub mod middleware;
pub mod routes;

use routes::api_routes;

#[tokio::main]
pub async fn start() {
    // Combine all routes for v1 and v2
    let api_routes = api_routes();

    // Start the server on port 3030
    println!("Server running on http://127.0.0.1:3030");
    warp::serve(api_routes).run(([127, 0, 0, 1], 3030)).await;
}
