use std::{convert::Infallible, future::Future};

use axum::{
    extract::{Request, State},
    handler::Handler,
    middleware::{from_fn_with_state, Next},
    response::IntoResponse,
    routing::{delete, get, head, options, patch, post, put, trace, MethodRouter, Route},
    Router,
};
use tower::{Layer, Service};

use crate::NamedRoutesRepo;

/// Axum Router Wrapper
/// Instead of using Axum's default Router to register routes
/// this type should be used
///
/// ```rust
///  use named_routes_axum::RouterWrapper;
///
/// let app = axum::Router::<()>::new();
///
/// let my_named_routes = RouterWrapper::new().get("/hello", || async { "Hello world" }, "index-page");
///
/// app.merge(my_named_routes.into_router()); // then get the actual axum router built
///
/// ```
#[derive(Debug, Clone)]
pub struct RouterWrapper<S = ()> {
    router: Router<S>,
    name_repo: NamedRoutesRepo,
}

impl<S: Clone + Send + Sync + 'static> Default for RouterWrapper<S> {
    fn default() -> Self {
        Self {
            router: Router::new(),
            name_repo: Default::default(),
        }
    }
}

impl<S: Clone + Send + Sync + 'static> RouterWrapper<S> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a DELETE handler
    pub fn delete<H, T>(self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_route(path, delete(handler), name)
    }

    /// Register a DELETE handler with no name
    pub fn delete_x<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.route(path, delete(handler))
    }

    /// Register a GET handler
    pub fn get<H, T>(self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_route(path, get(handler), name)
    }

    /// Register a GET handler with no name
    pub fn get_x<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.route(path, get(handler))
    }

    /// Register a HEAD handler
    pub fn head<H, T>(self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_route(path, head(handler), name)
    }

    /// Register a HEAD handler with no name
    pub fn head_x<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.route(path, head(handler))
    }

    /// Register a OPTIONS handler
    pub fn options<H, T>(self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_route(path, options(handler), name)
    }

    /// Register a OPTIONS handler with no name
    pub fn options_x<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.route(path, options(handler))
    }

    /// Register a PATCH handler
    pub fn patch<H, T>(self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_route(path, patch(handler), name)
    }

    /// Register a PATCH handler with no name
    pub fn patch_x<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.route(path, patch(handler))
    }

    /// Register a POST handler
    pub fn post<H, T>(self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_route(path, post(handler), name)
    }

    /// Register a POST handler with no name
    pub fn post_x<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.route(path, post(handler))
    }

    /// Register a PUT handler
    pub fn put<H, T>(self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_route(path, put(handler), name)
    }

    /// Register a PUT handler with no name
    pub fn put_x<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.route(path, put(handler))
    }

    /// Register a TRACE handler
    pub fn trace<H, T>(self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_route(path, trace(handler), name)
    }

    /// Register a TRACE handler with no name
    pub fn trace_x<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.route(path, trace(handler))
    }

    /// Register a named route handler that handles most of the common HTTP verbs:
    ///  - GET, POST, PUT, DELETE, PATCH , OPTIONS, TRACE
    pub fn any<H, T>(self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_route(
            path,
            get(handler.clone())
                .post(handler.clone())
                .put(handler.clone())
                .delete(handler.clone())
                .patch(handler.clone())
                .options(handler.clone())
                .trace(handler.clone()),
            name,
        )
    }

    /// Register a route handler that handles most of the common HTTP verbs:
    ///  - GET, POST, PUT, DELETE, PATCH , OPTIONS, TRACE
    pub fn any_x<H, T>(self, path: &str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.route(
            path,
            get(handler.clone())
                .post(handler.clone())
                .put(handler.clone())
                .delete(handler.clone())
                .patch(handler.clone())
                .options(handler.clone())
                .trace(handler.clone()),
        )
    }

    /// Register a named route handler that handles one or more HTTP verbs:
    pub fn any_of<H, T, V>(self, verbs: &[V], path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
        V: ToString,
    {
        if verbs.len() == 0 {
            return self;
        }
        let list = self.build_verb_list(verbs, handler);

        self.name_route(path, list, name)
    }

    /// Register a route handler that handles one or more HTTP verbs:
    pub fn any_of_x<H, T, V>(self, verbs: &[V], path: &str, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
        V: ToString,
    {
        if verbs.len() == 0 {
            return self;
        }
        let list = self.build_verb_list(verbs, handler);

        self.route(path, list)
    }

    pub fn route(mut self, path: &str, handler: MethodRouter<S>) -> Self {
        self.router = self.router.route(path, handler);
        self
    }

    pub fn name_route(mut self, path: &str, handler: MethodRouter<S>, name: &str) -> Self {
        self.name_repo = self.name_repo.register(name, path);
        self.route(path, handler)
    }

    pub fn merge(mut self, wrapper: Self) -> Self {
        self.router = self.router.merge(wrapper.into_router());
        self
    }

    /// An new instance of this struct will be passed to the provided callback
    pub fn merge_given<C>(self, mut callback: C) -> Self
    where
        C: FnMut(Self) -> Self,
    {
        self.merge(callback(Self::new()))
    }

    pub fn nest(mut self, path: &str, wrapper: Self) -> Self {
        self.router = self.router.nest(path, wrapper.into_router());
        self
    }

    /// An new instance of this struct will be passed to the provided callback
    pub fn nest_given<C>(self, path: &str, mut callback: C) -> Self
    where
        C: FnMut(Self) -> Self,
    {
        self.nest(path, callback(Self::new()))
    }

    pub fn middleware<F, Fut, Out>(mut self, f: F) -> Self
    where
        F: FnMut(Request, Next) -> Fut + Clone + Send + 'static,
        Fut: Future<Output = Out> + Send + 'static,
        Out: IntoResponse + 'static,
    {
        self.router = self.router.route_layer(from_fn_with_state((), f));

        self
    }

    /// Register tower's layer service
    pub fn layer<L>(mut self, layer: L) -> Self
    where
        L: Layer<Route> + Clone + Send + 'static,
        L::Service: Service<Request> + Clone + Send + 'static,
        <L::Service as Service<Request>>::Response: IntoResponse + 'static,
        <L::Service as Service<Request>>::Error: Into<Infallible> + 'static,
        <L::Service as Service<Request>>::Future: Send + 'static,
    {
        self.router = self.router.layer(layer);
        self
    }

    /// Register tower's router layer service
    pub fn route_layer<L>(mut self, layer: L) -> Self
    where
        L: Layer<Route> + Clone + Send + 'static,
        L::Service: Service<Request> + Clone + Send + 'static,
        <L::Service as Service<Request>>::Response: IntoResponse + 'static,
        <L::Service as Service<Request>>::Error: Into<Infallible> + 'static,
        <L::Service as Service<Request>>::Future: Send + 'static,
    {
        self.router = self.router.route_layer(layer);
        self
    }

    pub fn middleware_with_state<F, Fut, Out, ST>(mut self, f: F, state: ST) -> Self
    where
        F: FnMut(State<ST>, Request, Next) -> Fut + Clone + Send + 'static,
        Fut: Future<Output = Out> + Send + 'static,
        Out: IntoResponse + 'static,
        ST: Clone + Send + Sync + 'static,
    {
        self.router = self.router.route_layer(from_fn_with_state(state, f));

        self
    }

    /// Returns the Axum Router instance
    pub fn into_router(self) -> Router<S> {
        self.router
    }

    pub fn build_verb_list<H, T, V>(&self, verbs: &[V], handler: H) -> MethodRouter<S>
    where
        H: Handler<T, S>,
        T: 'static,
        V: ToString,
    {
        let mut list: MethodRouter<S> = MethodRouter::new();

        for entry in verbs {
            list = match entry.to_string().trim().to_ascii_uppercase().as_str() {
                "GET" => list.get(handler.clone()),
                "POST" => list.post(handler.clone()),
                "PUT" => list.put(handler.clone()),
                "DELETE" => list.delete(handler.clone()),
                "PATCH" => list.patch(handler.clone()),
                "OPTION" | "OPTIONS" => list.options(handler.clone()),
                "TRACE" => list.trace(handler.clone()),
                _ => list,
            };
        }

        list
    }
}

impl<S> From<Router<S>> for RouterWrapper<S> {
    fn from(router: Router<S>) -> Self {
        Self {
            router,
            name_repo: Default::default(),
        }
    }
}
