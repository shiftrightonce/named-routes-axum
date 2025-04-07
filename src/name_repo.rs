use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{RepoInner, NAME_ROUTES_REPO};

#[derive(Debug, Clone)]
pub struct NamedRoutesRepo {
    prefix: Option<String>,
    repo: RepoInner,
}

impl Default for NamedRoutesRepo {
    fn default() -> Self {
        Self {
            prefix: None,
            repo: Arc::clone(
                NAME_ROUTES_REPO.get_or_init(|| Arc::new(RwLock::new(HashMap::new()))),
            ),
        }
    }
}

impl NamedRoutesRepo {
    pub fn new(prefix: Option<&str>) -> Self {
        Self {
            prefix: prefix.map(|p| p.to_string()),

            repo: Arc::clone(
                NAME_ROUTES_REPO.get_or_init(|| Arc::new(RwLock::new(HashMap::new()))),
            ),
        }
    }
    pub fn register(&self, name: &str, url: &str) -> &Self {
        if let Ok(mut write_lock) = self.repo.write() {
            if self.prefix.is_some() {
                write_lock.insert(
                    name.to_string(),
                    format!("{}{}", self.prefix.as_ref().unwrap(), url).into(),
                );
            } else {
                write_lock.insert(name.to_string(), url.into());
            }
        }

        self
    }

    pub(crate) fn build_child_prefix(&self, child_prefix: &str) -> String {
        let parent = if let Some(p) = self.prefix.as_ref() {
            p.as_str()
        } else {
            ""
        };

        format!("{}{}", parent, child_prefix)
    }
}
