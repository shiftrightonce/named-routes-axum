use axum::{extract::Request, middleware::Next, response::IntoResponse};
use named_routes_axum::RouterWrapper;

#[tokio::main]
async fn main() {
    let app = RouterWrapper::new()
        .get("/", handle_index, "index")
        .middleware(|req, next| async {
            eprintln!("in middleware 1");
            next.run(req).await
        })
        .middleware(middleware_two);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_router()).await.unwrap();
}

async fn handle_index() -> impl IntoResponse {
    "Index page"
}

async fn middleware_two(req: Request, next: Next) -> impl IntoResponse {
    eprintln!("in middleware 2");
    next.run(req).await
}
