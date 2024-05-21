use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use named_routes_axum::{NamedRoutesService, RouterWrapper};

#[tokio::main]
async fn main() {
    // 1. Application state
    let state = AppState::default();

    // 2.  build our application with a route
    let app = RouterWrapper::new().get("/", handler, "home").get(
        "/add/:number1/:number2",
        handle_adding,
        "add_numbers",
    );

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_router().with_state(state))
        .await
        .unwrap();
}

async fn handler(State(app): State<AppState>) -> impl IntoResponse {
    // 3. Get the route with the name "add_numbers" and redirect to it
    if let Some(route) = app.route_service().get("add_numbers") {
        // 4. The route named "add_numbers" has two parts. These are the values
        let mut parts = HashMap::new();
        parts.insert("number1", 1);
        parts.insert("number2", 2);

        return route.with(parts).redirect(Html("")); // we are creating a response with an empty HTML body
    } else {
        Html("<h1>We could not get the route named <b>add_numbers</b></h1>").into_response()
    }
}

async fn handle_adding(Path((number1, number2)): Path<(i32, i32)>) -> Html<String> {
    Html(format!("{} + {} = {}", number1, number2, number1 + number2))
}

#[derive(Debug, Default, Clone)]
struct AppState {
    route_service: NamedRoutesService,
}

impl AppState {
    fn route_service(&self) -> &NamedRoutesService {
        &self.route_service
    }
}
