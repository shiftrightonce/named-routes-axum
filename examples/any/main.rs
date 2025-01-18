use axum::{
    extract::{Path, Request},
    http::{header::CONTENT_TYPE, Response},
    response::{Html, IntoResponse},
};
use named_routes_axum::RouterWrapper;

#[tokio::main]
async fn main() {
    // 1. Setup two routes that will handle all of the most common HTTP's verbs
    let app = RouterWrapper::new().any_x("/", index_page).any(
        "/x/{id}",
        handle_any_verb_for_x,
        "x-resource",
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_router()).await.unwrap();
}

async fn index_page() -> impl IntoResponse {
    Html("Server for Resource X".to_string())
}

async fn handle_any_verb_for_x(Path(id): Path<i32>, req: Request) -> impl IntoResponse {
    let data = format!("{{\"method\": \"{}\", \"id\": {}}}", req.method(), id);
    let mut response = Response::new(data);
    response
        .headers_mut()
        .insert(CONTENT_TYPE, "application/json".parse().unwrap());

    response
}
