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
#[cfg(test)]
mod test {
    use crate::{NamedRoutesRepo, NamedRoutesService};

    const HOME_URL: (&'static str, &'static str) = ("home", "/");
    const URL2: (&'static str, &'static str) = ("url_2", "/first");
    const URL3: (&'static str, &'static str) = ("url_3", "/second");
    const URL4: (&'static str, &'static str) = ("url_4", "/four/{path1}/{path2}");
    const URL5: (&'static str, &'static str) = ("url_5", "/four/{path1}/{path2}/user/{path3}");

    #[allow(unused)]
    fn setup_name_repo() {
        NamedRoutesRepo::default()
            .register(HOME_URL.0, &HOME_URL.1)
            .register(URL2.0, &URL2.1)
            .register(URL3.0, &URL3.1)
            .register(URL4.0, &URL4.1)
            .register(URL5.0, &URL5.1);
    }

    #[test]
    fn test_registering() {
        setup_name_repo();
        let name_service = NamedRoutesService::new();

        // success tests
        assert_eq!(
            name_service.get_path(HOME_URL.0),
            Some(HOME_URL.1.to_string()),
        );
        assert_eq!(name_service.get_path(URL2.0), Some(URL2.1.to_owned()),);
        assert_eq!(name_service.get_path(URL3.0), Some(URL3.1.to_string()),);

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
        assert_eq!(
            name_service.get_path(HOME_URL.0),
            Some(HOME_URL.1.to_string())
        );
        assert_eq!(name_service.get_path(URL2.0), Some(URL2.1.to_string()));
        assert_eq!(name_service.get_path(URL3.0), Some(URL3.1.to_string()));
        assert_eq!(name_service.get_path("second-foo"), None);
    }

    #[test]
    fn test_getting_all_the_names() {
        setup_name_repo();
        let name_service = NamedRoutesService::new();
        let all = name_service.all();

        // success
        assert_eq!(all.len(), 5);
    }

    #[test]
    fn test_we_are_using_one_repo() {
        setup_name_repo();
        let name_service2 = NamedRoutesService::new();

        // success test
        assert_eq!(
            name_service2.get_path(HOME_URL.0),
            Some(HOME_URL.1.to_string())
        );
        assert_eq!(name_service2.get_path(URL2.0), Some(URL2.1.to_string()));
        assert_eq!(name_service2.get_path(URL3.0), Some(URL3.1.to_string()));
        assert_eq!(name_service2.get_path("second-foo"), None);

        // fail test
        assert_eq!(name_service2.get_path("home2"), None);
    }
}
