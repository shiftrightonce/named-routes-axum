//! # Named Routes Axum is a crate that allows you to name your axum routes
//!
//!
//! ```rust
//!  use named_routes_axum::{RouterWrapper, NamedRoutesService};
//!
//! let app = axum::Router::<()>::new();
//!
//! let my_named_routes = RouterWrapper::new().get("/hello", || async { "Hello world" }, "index-page");
//!
//! app.merge(my_named_routes.into_router()); // then get the actual axum router built
//!
//! // using  an instance of the `NamedRoutesService` you can redirect or get the route path
//!
//! let path = NamedRoutesService::new().get("index-page").unwrap().redirector().path();
//!
//!
//! ```
//!
mod name_repo;
mod redirector;
mod route_path;
mod router_wrapper;
mod service;

pub mod helpers;

pub(crate) static NAME_ROUTES_REPO: OnceLock<RepoInner> = OnceLock::new();
pub(crate) type RepoInner = Arc<RwLock<HashMap<String, RoutePath>>>;

use std::{
    collections::HashMap,
    sync::{Arc, OnceLock, RwLock},
};

pub(crate) use name_repo::*;

pub use route_path::*;
pub use router_wrapper::*;
pub use service::NamedRoutesService;
