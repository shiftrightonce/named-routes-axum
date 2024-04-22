use std::{
    collections::HashMap,
    sync::{Arc, OnceLock, RwLock},
};

use crate::route_path::RoutePath;

type RepoInner = Arc<RwLock<HashMap<String, RoutePath>>>;
static NAME_ROUTES_REPO: OnceLock<RepoInner> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct NameRepo {
    repo: RepoInner,
}

impl Default for NameRepo {
    fn default() -> Self {
        Self {
            repo: Arc::clone(NAME_ROUTES_REPO.get_or_init(|| {
                return Arc::new(RwLock::new(HashMap::new()));
            })),
        }
    }
}

impl NameRepo {
    fn new() -> Self {
        Self::default()
    }

    pub fn register(self, name: &str, url: &str) -> Self {
        if let Ok(mut write_lock) = self.repo.write() {
            write_lock.insert(name.to_string(), url.into());
        }
        self
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
    use super::*;

    #[allow(unused)]
    fn setup_name_repo() -> NameRepo {
        let home_url = "/".to_string();
        let first_url = "/first".to_string();
        let second_url = "/second".to_string();

        NameRepo::new()
            .register("home", &home_url)
            .register("first", &first_url)
            .register("second", &second_url)
    }

    #[allow(unused)]
    fn setup_name_repo_with_repo(repo: RepoInner) -> NameRepo {
        let home_url = "/".to_string();
        let first_url = "/first".to_string();
        let second_url = "/second".to_string();

        NameRepo::new()
            .set_repo(repo)
            .register("home", &home_url)
            .register("first", &first_url)
            .register("second", &second_url)
    }

    #[test]
    fn test_registering() {
        let home_url = "/".to_string();
        let first_url = "/first".to_string();
        let second_url = "/second".to_string();

        let name_repo = setup_name_repo();

        // success tests
        assert_eq!(
            name_repo.get_string("home"),
            Some(home_url.clone()),
            "expected /home"
        );
        assert_eq!(
            name_repo.get_string("first"),
            Some(first_url.clone()),
            "expected /first"
        );
        assert_eq!(
            name_repo.get_string("second"),
            Some(second_url.clone()),
            "expected /second"
        );

        // fail tests
        assert_eq!(
            name_repo.get_string("fake1"),
            None,
            "fake1 was not registered"
        );
        assert_eq!(name_repo.get_string("home-"), None, "home- has a typo");
        assert_eq!(
            name_repo.get_string("first route"),
            None,
            "first route has a space"
        );
    }

    #[test]
    fn test_getting_url() {
        let name_repo = setup_name_repo();

        // success test
        assert_eq!(name_repo.get_string("home"), Some("/".to_string()));
        assert_eq!(name_repo.get_string("first"), Some("/first".to_string()));
        assert_eq!(name_repo.get_string("second"), Some("/second".to_string()));
        assert_eq!(name_repo.get_string("second-foo"), None);
    }

    #[test]
    fn test_getting_all_the_names() {
        let name_repo = setup_name_repo();
        let all = name_repo.all_as_string();

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
        _ = setup_name_repo();
        let mut name_repo2 = NameRepo::new();

        // success test
        assert_eq!(name_repo2.get_string("home"), Some("/".to_string()));
        assert_eq!(name_repo2.get_string("first"), Some("/first".to_string()));
        assert_eq!(name_repo2.get_string("second"), Some("/second".to_string()));
        assert_eq!(name_repo2.get_string("second-foo"), None);

        name_repo2 = name_repo2.register("user-age", "/user/age");
        assert_eq!(
            name_repo2.get_string("user-age"),
            Some("/user/age".to_string())
        );
        assert_eq!(name_repo2.get_string("second-foo2"), None);

        // fail test
        assert_eq!(name_repo2.get_string("home2"), None);
    }

    #[test]
    fn test_has_method() {
        let mut name_repo = setup_name_repo_with_repo(Arc::default());

        // success tests
        assert_eq!(name_repo.has("foobar"), false);
        assert_eq!(name_repo.has("home3"), false);

        name_repo = name_repo.register("foobar-exist", "/foobar-exist");
        assert_eq!(name_repo.has("foobar-exist"), true);

        // fail tests
        assert_eq!(name_repo.has("foobar-exist"), true);
        assert_eq!(name_repo.has("foobar-does-not"), false);
    }
}
