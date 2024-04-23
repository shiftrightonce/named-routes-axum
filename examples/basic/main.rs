use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use named_routes_axum::{NamedRoutesService, RouterWrapper};

#[tokio::main]
async fn main() {
    let state = AppState::default();

    // build our application with a route
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
    if let Some(mut route) = app.route_service().get("add_numbers") {
        route.set_param("number1", 8).set_param("number2", 55);
        return route.redirect(Html("Redirecting"));
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
