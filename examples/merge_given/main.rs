use axum::response::IntoResponse;
use named_routes_axum::RouterWrapper;

#[tokio::main]
async fn main() {
    let app = RouterWrapper::new()
        .get("/", home_handler, "home")
        .merge_given(|router| router.get("/two", route_two_handler, "route-two"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_router()).await.unwrap();
}

async fn home_handler() -> impl IntoResponse {
    "Welcome"
}

async fn route_two_handler() -> impl IntoResponse {
    "Hello from the second route"
}
