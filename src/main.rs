use core::num;
use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Html, IntoResponse, Response},
    routing::{get, Route},
    Router,
};
use busybody::helpers::get_service;
use name_repo::NameRepo;
use router_wrapper::RouterWrapper;

mod name_repo;
mod route_path;
mod router_wrapper;

#[tokio::main]
async fn main() {
    let state = AppState::default();

    busybody::helpers::register_service(state);

    // build our application with a route
    let mut app = register_routes();
    let some_other_router = RouterWrapper::new()
        .get("/hello2", say_hello2, "hello2")
        .get("/add/:number/:number2", adder, "adder");
    let some_other_router2 = RouterWrapper::<()>::new().get("/hello22", say_hello2, "hello22");

    app = app.merge(some_other_router.into_router());

    println!(
        "say_hello path2: {:#?}",
        some_other_router2.repo().get("say_hello")
    );

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

fn register_routes() -> Router<()> {
    let router2 = Router::new().route("/two-2", get(handler2));
    let wrapped_router = RouterWrapper::new().get("/hello", say_hello, "say_hello");

    println!(
        "say_hello path: {:#?}",
        wrapped_router.repo().get("say_hello")
    );

    Router::new()
        .route("/", get(handler))
        .route("/two", get(handler2))
        .merge(router2)
        .merge(wrapped_router.into_router())
}

async fn adder(Path(number1): Path<i32>, Path(number2): Path<i32>) -> impl IntoResponse {
    format!("{} + {} = {}", number1, number2, number1 + number2)
}

async fn say_hello() -> Html<&'static str> {
    Html("Hello from our wrapped router")
}

async fn say_hello2() -> Html<&'static str> {
    Html("<h1>Hello from our wrapped router</h2>")
}

async fn handler() -> Response<String> {
    let app_state = get_service::<AppState>().unwrap();
    let route = app_state.name_repo.get("hello2");
    dbg!(&route);
    // TODO: Get a route by it's name

    route.unwrap().redirect_t("Hello world".to_string())
    // if true {
    //     let content = format!(
    //         "<html>
    //         {}
    //     </html>",
    //         route.unwrap().redirect_meta()
    //     );
    //     Html(content)
    // } else {
    //     Html("<h1>Hello everyone from handler 1</h1>".to_string())
    // }
}

async fn handler2() -> Html<&'static str> {
    Html("<h1>Hello everyone from handler 2</h1>")
}

#[derive(Debug, Default, Clone)]
struct AppState {
    name_repo: NameRepo,
}

impl AppState {
    fn new(name_repo: NameRepo) -> Self {
        Self { name_repo }
    }
}
