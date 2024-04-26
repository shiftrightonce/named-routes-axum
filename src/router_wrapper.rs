use axum::{
    handler::Handler,
    routing::{delete, get, head, options, patch, post, put, trace, MethodRouter},
    Router,
};

use crate::NamedRoutesRepo;

/// Axum Router Wrapper
/// Instead of using Axum's default Router to register routes
/// this type schould be used
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
    pub fn delete<H, T>(mut self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_repo = self.name_repo.register(name, path);
        self.router = self.router.route(path, delete(handler));
        self
    }

    /// Register a GET handler
    pub fn get<H, T>(mut self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_repo = self.name_repo.register(name, path);
        self.router = self.router.route(path, get(handler));
        self
    }

    /// Register a HEAD handler
    pub fn head<H, T>(mut self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_repo = self.name_repo.register(name, path);
        self.router = self.router.route(path, head(handler));
        self
    }

    /// Register a OTPIONS handler
    pub fn opitons<H, T>(mut self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_repo = self.name_repo.register(name, path);
        self.router = self.router.route(path, options(handler));
        self
    }

    /// Register a PATCH handler
    pub fn patch<H, T>(mut self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_repo = self.name_repo.register(name, path);
        self.router = self.router.route(path, patch(handler));
        self
    }

    /// Register a POST handler
    pub fn post<H, T>(mut self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_repo = self.name_repo.register(name, path);
        self.router = self.router.route(path, post(handler));
        self
    }

    /// Register a PUT handler
    pub fn put<H, T>(mut self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_repo = self.name_repo.register(name, path);
        self.router = self.router.route(path, put(handler));
        self
    }

    /// Register a TRACE handler
    pub fn trace<H, T>(mut self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_repo = self.name_repo.register(name, path);
        self.router = self.router.route(path, trace(handler));
        self
    }

    pub fn route(mut self, path: &str, handler: MethodRouter<S>) -> Self {
        self.router = self.router.route(path, handler);
        self
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
