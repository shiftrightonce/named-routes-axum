use axum::{
    handler::Handler,
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

    /// Register a GET handler
    pub fn get<H, T>(self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_route(path, get(handler), name)
    }

    /// Register a HEAD handler
    pub fn head<H, T>(self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_route(path, head(handler), name)
    }

    /// Register a OPTIONS handler
    pub fn options<H, T>(self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_route(path, options(handler), name)
    }

    /// Register a PATCH handler
    pub fn patch<H, T>(self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_route(path, patch(handler), name)
    }

    /// Register a POST handler
    pub fn post<H, T>(self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_route(path, post(handler), name)
    }

    /// Register a PUT handler
    pub fn put<H, T>(self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_route(path, put(handler), name)
    }

    /// Register a TRACE handler
    pub fn trace<H, T>(self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_route(path, trace(handler), name)
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

    /// Returns the Axum Router instance
    pub fn into_router(self) -> Router<S> {
        self.router
    }
}
