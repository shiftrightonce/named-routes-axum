use axum::{response::IntoResponse, routing::get};
use named_routes_axum::RouterWrapper;

#[tokio::main]
async fn main() {
    let app = RouterWrapper::new()
        .get("/", home_handler, "home") // 1. The home route has a name
        .route("/no-name", get(no_name_route)) // 2. This route does not have a name
        .name_route("/awesome", get(awesome_route), "awesome"); // 3. Another named route

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_router()).await.unwrap();
}

async fn home_handler() -> impl IntoResponse {
    "Welcome"
}

async fn no_name_route() -> impl IntoResponse {
    "This route does not have a name"
}

async fn awesome_route() -> impl IntoResponse {
    "Hello from the awesome route"
}
