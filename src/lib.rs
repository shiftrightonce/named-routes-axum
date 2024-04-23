mod name_repo;
mod route_path;
mod router_wrapper;
mod service;

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
