use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
};
use named_routes_axum::{NamedRoutesService, RouterWrapper};
use rand::Rng;

#[tokio::main]
async fn main() {
    // 1. Application state
    let state = AppState::default();

    // 2.  build our application with a route
    let app = RouterWrapper::new()
        .get("/", handler, "home")
        .get("/day/:index", handle_day, "day");

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
    // 3. Get the route with name "day" and redirect to it
    if let Some(route) = app.route_service().get("day") {
        // 4. The route named "day" requires an i32 value
        // these are the values.
        let part = rand::thread_rng().gen_range(0..6);

        return route.with(part.to_string()).redirect(Html("")); // we are creating a response with an empty HTML body
    } else {
        Html("<h1>We could not get the route named <b>add_numbers</b></h1>").into_response()
    }
}

async fn handle_day(Path(index): Path<i32>) -> Html<&'static str> {
    let days = [
        "Sunday",
        "Monday",
        "Tuesday",
        "Wednesday",
        "Thursday",
        "Friday",
        "Saturday",
    ];
    let unknown = "Unknown";

    Html(days.get(index as usize).unwrap_or(&unknown))
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
