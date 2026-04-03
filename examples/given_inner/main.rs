use axum::{response::Html, routing::get};
use named_routes_axum::RouterWrapper;

#[tokio::main]
async fn main() {
    // `given_inner` provides the inner axum router
    let app = RouterWrapper::new().given_inner(|axum_router| {
        axum_router
            .route("/", get(|| async { Html("<h1>Index Page</h1>") }))
            .route("/about-us", get(|| async { Html("<h1>About Us</h1>") }))
    });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_router()).await.unwrap();
}
