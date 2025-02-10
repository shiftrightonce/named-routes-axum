# Named Routes Axum 

`Name Routes Axums` is a crate that allows you to name your axum routes


 ```rust
  use named_routes_axum::{RouterWrapper, NamedRoutesService};

 let app = axum::Router::<()>::new();
 let my_named_routes = RouterWrapper::new().get("/hello", || async { "Hello world" }, "index-page");

app.merge(my_named_routes.into_router()); // then get the actual axum router built
// using  an instance of the `NamedRoutesService` you can redirect or get the route path

let path = NamedRoutesService::new().get("index-page").unwrap().redirector().path();
 ```

<details>
  <summary>
     Full Example
  </summary>
  
  ```rust
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
        .get("/day/{index}", handle_day, "day");

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
        // 4. The route named "day" requires a value
        let part = rand::rng().random_range(0..6);

        return route.with(part.to_string()).redirect(Html("")); // we are creating a response with an empty HTML body

        // or
        // return route.with((part,)).redirect(Html("")); 
        // return route.with(vec![part]).redirect(Html("")); 
        //
        // let mut map = std::collections::HashMap::new();
        //  map.insert("index", part)
        // return route.with(map).redirect(Html("")); 
    } else {
        Html("<h1>We could not get the rout named <b>add_numbers</b></h1>").into_response()
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

  ```
</details>



## More Examples
The [examples](https://github.com/shiftrightonce/named-routes-axum/tree/main/examples) folder contains simple and full examples. If none of the examples are helpful,
please reach out with your use case and I  try to provide one.


## Feedback
If you find this crate useful, please star the repository. Submit your issues and recommendations as well.

## License

### The MIT License (MIT)

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.