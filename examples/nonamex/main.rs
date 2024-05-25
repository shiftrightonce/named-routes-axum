use axum::response::IntoResponse;
use named_routes_axum::RouterWrapper;

#[tokio::main]
async fn main() {
    let app = RouterWrapper::new()
        .get("/", home_handler, "home") // 1. The home route has a name
        // 2. This route does not have a name
        //    All the verbs have an _x method that allows you to not include a name
        //    Below is an example of `get`'s `_x` version
        .get_x("/no-name", no_name_route);

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
