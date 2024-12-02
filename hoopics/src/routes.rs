use warp::Filter;

pub fn v1_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Define GET /v1/hello
    let hello_v1 = warp::path!("v1" / "hello")
        .map(|| warp::reply::json(&"Hello from v1!"));

    // Define POST /v1/data
    let data_v1 = warp::path!("v1" / "data")
        .and(warp::post())
        .and(warp::body::json())
        .map(|body: serde_json::Value| {
            // Process the incoming JSON data
            warp::reply::json(&body)
        });

    // Combine routes
    hello_v1.or(data_v1)
}

pub fn v2_routes() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Define GET /v2/hello
    let hello_v2 = warp::path!("v2" / "hello")
        .map(|| warp::reply::json(&"Hello from v2!"));

    // Define POST /v2/data
    let data_v2 = warp::path!("v2" / "data")
        .and(warp::post())
        .and(warp::body::json())
        .map(|body: serde_json::Value| {
            // Process the incoming JSON data
            warp::reply::json(&body)
        });

    // Combine routes
    hello_v2.or(data_v2)
}