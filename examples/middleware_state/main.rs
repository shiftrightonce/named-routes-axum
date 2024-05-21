use std::sync::{atomic::AtomicI64, Arc};

use axum::{
    extract::{Request, State},
    middleware::Next,
    response::IntoResponse,
    Extension,
};
use named_routes_axum::RouterWrapper;

#[tokio::main]
async fn main() {
    let middleware_state = MiddlewareState::default();

    let app = RouterWrapper::new()
        // 1. Homepage route
        .get("/", handle_index, "index")
        // 2. Middleware comes after index route therefore it affects all the routes before
        .middleware_with_state(
            |State(counter): State<MiddlewareState>, req, next| async move {
                eprintln!("in middleware 1. Counting the number of visitors hitting the homepage");
                counter.inc(); // 3. Increment the visitor count visting the index page
                next.run(req).await
            },
            middleware_state.clone(),
        )
        // 4. Disply the total visittors so far
        .get("/total", handle_get_count, "get-total")
        // 5. `middleware_two`  provides the middle's state as an "extension"
        .middleware_with_state(middleware_two, middleware_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_router()).await.unwrap();
}

async fn handle_index() -> impl IntoResponse {
    "Hello to my awesome site"
}

async fn handle_get_count(Extension(counter): Extension<MiddlewareState>) -> impl IntoResponse {
    format!("Total visitors so far: {}", counter.total())
}

async fn middleware_two(
    State(counter): State<MiddlewareState>,
    mut req: Request,
    next: Next,
) -> impl IntoResponse {
    let total = counter.total();
    eprintln!("in middleware 2. Total visitor counted: {}", total);

    req.extensions_mut().insert(counter); // 6. Insert the counter as an extension for the route handlers to consume

    next.run(req).await
}

#[derive(Default, Clone)]
struct MiddlewareState(Arc<AtomicI64>);

impl MiddlewareState {
    pub fn inc(&self) -> i64 {
        self.0.fetch_add(1, std::sync::atomic::Ordering::AcqRel)
    }

    pub fn total(&self) -> i64 {
        self.0.load(std::sync::atomic::Ordering::Relaxed)
    }
}
