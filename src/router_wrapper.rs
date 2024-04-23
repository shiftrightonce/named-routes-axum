use axum::{
    handler::Handler,
    routing::{get, post, put},
    Router,
};

use crate::NamedRoutesRepo;

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

    pub fn get<H, T>(mut self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_repo = self.name_repo.register(name, path);
        self.router = self.router.route(path, get(handler));
        self
    }

    pub fn post<H, T>(mut self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_repo = self.name_repo.register(name, path);
        self.router = self.router.route(path, post(handler));
        self
    }

    pub fn put<H, T>(mut self, path: &str, handler: H, name: &str) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        self.name_repo = self.name_repo.register(name, path);
        self.router = self.router.route(path, put(handler));
        self
    }

    // TODO: Remove this method!!! when done with the POE
    pub fn repo(&self) -> &NamedRoutesRepo {
        &self.name_repo
    }

    pub fn into_router(self) -> Router<S> {
        self.router
    }
}
