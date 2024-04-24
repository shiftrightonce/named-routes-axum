use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use named_routes_axum::{NamedRoutesService, RouterWrapper};

#[tokio::main]
async fn main() {
    let state = AppState::default();

    // build our application with a route
    let app = RouterWrapper::new()
        .get("/", handler, "home")
        .get("/add/:number1/:number2", handle_adding, "add_numbers")
        .get("/dummy", dummy, "dummy");

    let router2 = RouterWrapper::new().get(
        "/foo",
        |State(app): State<AppState>| async move {
            let mut parts = HashMap::new();
            parts.insert("number1", 1);
            parts.insert("number2", 1);

            let path = app
                .route_service()
                .get("add_numbers")
                .unwrap()
                .with(parts)
                .path();

            format!("add path is: {}", path)
        },
        "foo-route",
    );

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        app.into_router()
            .merge(router2.into_router())
            .with_state(state),
    )
    .await
    .unwrap();
}

async fn dummy(State(app): State<AppState>) -> impl IntoResponse {
    app.route_service().get("home").unwrap().redirect(())
}

async fn handler(State(app): State<AppState>) -> impl IntoResponse {
    if let Some(route) = app.route_service().get("add_numbers") {
        return route.with(vec![400, 1000]).redirect(Html("Redirecting"));
    }

    Html("<h1>Hello world</h1>").into_response()
}

async fn handle_adding(Path((number1, number2)): Path<(i32, i32)>) -> Html<String> {
    Html(format!(
        "<h1>{} + {} = {}</h1>",
        number1,
        number2,
        number1 + number2
    ))
}

#[derive(Debug, Default, Clone)]
struct AppState {
    route_service: NamedRoutesService,
}

impl AppState {
    fn new(route_service: NamedRoutesService) -> Self {
        Self { route_service }
    }

    fn route_service(&self) -> &NamedRoutesService {
        &self.route_service
    }
}
