use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use crate::{RepoInner, RoutePath, NAME_ROUTES_REPO};

#[derive(Debug, Clone)]
pub struct NamedRoutesService {
    repo: RepoInner,
}

impl Default for NamedRoutesService {
    fn default() -> Self {
        Self {
            repo: Arc::clone(NAME_ROUTES_REPO.get_or_init(|| {
                return Arc::new(RwLock::new(HashMap::new()));
            })),
        }
    }
}

impl NamedRoutesService {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, name: &str) -> Option<RoutePath> {
        if let Ok(read) = self.repo.read() {
            return if let Some(v) = read.get(name) {
                Some(v.clone())
            } else {
                None
            };
        } else {
            None
        }
    }

    pub fn get_string(&self, name: &str) -> Option<String> {
        self.get(name).map(|v| String::from(&v))
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

    pub fn all_as_string(&self) -> HashMap<String, String> {
        let mut values = HashMap::<String, String>::new();
        if let Ok(read) = self.repo.read() {
            for (k, v) in read.iter() {
                values.insert(k.clone(), v.into());
            }
            values
        } else {
            values
        }
    }

    fn set_repo(mut self, repo: RepoInner) -> Self {
        self.repo = repo;
        self
    }
}

mod test {
    use crate::NamedRoutesRepo;

    use super::*;

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
            name_service.get_string("home"),
            Some(home_url.clone()),
            "expected /home"
        );
        assert_eq!(
            name_service.get_string("first"),
            Some(first_url.clone()),
            "expected /first"
        );
        assert_eq!(
            name_service.get_string("second"),
            Some(second_url.clone()),
            "expected /second"
        );

        // fail tests
        assert_eq!(
            name_service.get_string("fake1"),
            None,
            "fake1 was not registered"
        );
        assert_eq!(name_service.get_string("home-"), None, "home- has a typo");
        assert_eq!(
            name_service.get_string("first route"),
            None,
            "first route has a space"
        );
    }

    #[test]
    fn test_getting_url() {
        setup_name_repo();
        let name_service = NamedRoutesService::new();

        // success test
        assert_eq!(name_service.get_string("home"), Some("/".to_string()));
        assert_eq!(name_service.get_string("first"), Some("/first".to_string()));
        assert_eq!(
            name_service.get_string("second"),
            Some("/second".to_string())
        );
        assert_eq!(name_service.get_string("second-foo"), None);
    }

    #[test]
    fn test_getting_all_the_names() {
        setup_name_repo();
        let name_service = NamedRoutesService::new();
        let all = name_service.all_as_string();

        // success
        assert_eq!(all.len(), 3);

        // success tests
        assert_eq!(all.get("home"), Some("/".to_string()).as_ref());
        assert_eq!(all.get("first"), Some("/first".to_string()).as_ref());
        assert_eq!(all.get("second"), Some("/second".to_string()).as_ref());
        assert_eq!(all.get("second-foo").take(), None);

        // fail tests
        assert_eq!(all.get("second-foo").take(), None);
        assert_eq!(all.get("").take(), None);
    }

    #[test]
    fn test_we_are_using_one_repo() {
        setup_name_repo();
        let name_service2 = NamedRoutesService::new();

        // success test
        assert_eq!(name_service2.get_string("home"), Some("/".to_string()));
        assert_eq!(
            name_service2.get_string("first"),
            Some("/first".to_string())
        );
        assert_eq!(
            name_service2.get_string("second"),
            Some("/second".to_string())
        );
        assert_eq!(name_service2.get_string("second-foo"), None);

        // fail test
        assert_eq!(name_service2.get_string("home2"), None);
    }
}
