use axum::{response::IntoResponse, routing::get, Router};
use named_routes_axum::RouterWrapper;

#[tokio::main]
async fn main() {
    // 1. A raw Axum router
    let raw_router = Router::new().route("/", get(handle_index));

    // 2. It is being converted into a `RouterWrapper`
    let mut app: RouterWrapper = RouterWrapper::from(raw_router);

    // 4. Calling some of the router wrapper's methods
    app = app.middleware(|req, next| async move {
        println!("middleware called");

        next.run(req).await
    });

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_router()).await.unwrap();
}

async fn handle_index() -> impl IntoResponse {
    "Hello from index page"
}
