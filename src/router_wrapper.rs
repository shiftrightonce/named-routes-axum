use std::future::Future;

use axum::{
    extract::{Request, State},
    handler::Handler,
    middleware::{from_fn_with_state, Next},
    response::IntoResponse,
    routing::{delete, get, head, options, patch, post, put, trace, MethodRouter},
    Router,
};

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
}

impl<S> From<Router<S>> for RouterWrapper<S> {
    fn from(router: Router<S>) -> Self {
        Self {
            router,
            name_repo: Default::default(),
        }
    }
}
