use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{RepoInner, NAME_ROUTES_REPO};

#[derive(Debug, Clone)]
pub struct NamedRoutesRepo {
    repo: RepoInner,
}

impl Default for NamedRoutesRepo {
    fn default() -> Self {
        Self {
            repo: Arc::clone(
                NAME_ROUTES_REPO.get_or_init(|| Arc::new(RwLock::new(HashMap::new()))),
            ),
        }
    }
}

impl NamedRoutesRepo {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(self, name: &str, url: &str) -> Self {
        if let Ok(mut write_lock) = self.repo.write() {
            write_lock.insert(name.to_string(), url.into());
        }
        self
    }
}
