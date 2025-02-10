use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{PartsValue, RepoInner, RoutePath, NAME_ROUTES_REPO};

#[derive(Debug, Clone)]
pub struct NamedRoutesService {
    repo: RepoInner,
}

impl Default for NamedRoutesService {
    fn default() -> Self {
        Self {
            repo: Arc::clone(
                NAME_ROUTES_REPO.get_or_init(|| Arc::new(RwLock::new(HashMap::new()))),
            ),
        }
    }
}

impl NamedRoutesService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, name: &str) -> Option<RoutePath> {
        if let Ok(read) = self.repo.read() {
            read.get(name).cloned()
        } else {
            None
        }
    }

    pub fn get_path(&self, name: &str) -> Option<String> {
        self.get(name).map(|v| v.redirector().path())
    }

    pub fn get_path_with<V: Into<PartsValue>>(&self, name: &str, parts: V) -> Option<String> {
        self.get(name).map(|v| v.with(parts).path())
    }

    pub fn has(&self, name: &str) -> bool {
        if let Ok(read) = self.repo.read() {
            read.contains_key(name)
        } else {
            false
        }
    }

    pub fn all(&self) -> HashMap<String, RoutePath> {
        if let Ok(read) = self.repo.read() {
            read.clone()
        } else {
            HashMap::new()
        }
    }
}

#[allow(unused_imports)]
mod test {
    use crate::{NamedRoutesRepo, NamedRoutesService};

    #[allow(unused)]
    fn setup_name_repo() {
        let home_url = "/".to_string();
        let first_url = "/first".to_string();
        let second_url = "/second".to_string();

        NamedRoutesRepo::new()
            .register("home", &home_url)
            .register("first", &first_url)
            .register("second", &second_url);
    }

    #[test]
    fn test_registering() {
        let home_url = "/".to_string();
        let first_url = "/first".to_string();
        let second_url = "/second".to_string();
        setup_name_repo();
        let name_service = NamedRoutesService::new();

        // success tests
        assert_eq!(
            name_service.get_path("home"),
            Some(home_url.clone()),
            "expected /home"
        );
        assert_eq!(
            name_service.get_path("first"),
            Some(first_url.clone()),
            "expected /first"
        );
        assert_eq!(
            name_service.get_path("second"),
            Some(second_url.clone()),
            "expected /second"
        );

        // fail tests
        assert_eq!(
            name_service.get_path("fake1"),
            None,
            "fake1 was not registered"
        );
        assert_eq!(name_service.get_path("home-"), None, "home- has a typo");
        assert_eq!(
            name_service.get_path("first route"),
            None,
            "first route has a space"
        );
    }

    #[test]
    fn test_getting_url() {
        setup_name_repo();
        let name_service = NamedRoutesService::new();

        // success test
        assert_eq!(name_service.get_path("home"), Some("/".to_string()));
        assert_eq!(name_service.get_path("first"), Some("/first".to_string()));
        assert_eq!(name_service.get_path("second"), Some("/second".to_string()));
        assert_eq!(name_service.get_path("second-foo"), None);
    }

    #[test]
    fn test_getting_all_the_names() {
        setup_name_repo();
        let name_service = NamedRoutesService::new();
        let all = name_service.all();

        // success
        assert_eq!(all.len(), 3);
    }

    #[test]
    fn test_we_are_using_one_repo() {
        setup_name_repo();
        let name_service2 = NamedRoutesService::new();

        // success test
        assert_eq!(name_service2.get_path("home"), Some("/".to_string()));
        assert_eq!(name_service2.get_path("first"), Some("/first".to_string()));
        assert_eq!(
            name_service2.get_path("second"),
            Some("/second".to_string())
        );
        assert_eq!(name_service2.get_path("second-foo"), None);

        // fail test
        assert_eq!(name_service2.get_path("home2"), None);
    }
}
