use warp::{Filter, Rejection, Reply};

mod routes;

#[tokio::main]
async fn main() {
    // Combine all routes for v1 and v2
    let api_routes = routes::v1_routes().or(routes::v2_routes());

    // Start the server on port 3030
    println!("Server running on http://127.0.0.1:3030");
    warp::serve(api_routes)
        .run(([127, 0, 0, 1], 3030))
        .await;
}