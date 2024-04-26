use std::collections::{BTreeMap, HashMap};

use axum::{
    body::Body,
    response::{IntoResponse, Response},
};

use crate::redirector::Redirector;

#[derive(Debug, Default, Clone)]
pub struct RoutePath {
    raw: String,
    has_parts: bool,
}

pub struct PartsValue {
    pos: Option<BTreeMap<usize, String>>,
    name: Option<HashMap<String, String>>,
}
impl RoutePath {
    pub fn with<P: Into<PartsValue>>(&self, values: P) -> Redirector {
        let parts = values.into();
        if parts.pos.is_some() {
            let map = parts.pos.unwrap();
            let mut named_map = HashMap::new();
            for (pos, name) in self.raw.split('/').filter(|p| p.contains(':')).enumerate() {
                let value = if let Some(v) = map.get(&pos) {
                    v.clone()
                } else {
                    "".to_string()
                };
                named_map.insert(name.to_string(), value);
            }

            self.make_redirector(Some(named_map))
        } else {
            self.make_redirector(Some(
                parts
                    .name
                    .unwrap()
                    .into_iter()
                    .map(|(k, v)| (format!(":{}", k), v.to_string()))
                    .collect(),
            ))
        }
    }

    pub fn has_parts(&self) -> bool {
        self.has_parts
    }

    pub fn redirect<T: IntoResponse>(&self, response: T) -> Response<Body> {
        self.make_redirector(None).redirect(response)
    }

    pub fn redirect_t<T>(&self, body: T) -> Response<T> {
        self.make_redirector(None).redirect_t(body)
    }

    pub fn redirect_meta(&self) -> String {
        self.make_redirector(None).redirect_meta()
    }

    pub fn redirector(&self) -> Redirector {
        self.make_redirector(None)
    }

    fn make_redirector(&self, parts: Option<HashMap<String, String>>) -> Redirector {
        Redirector::new(&self.raw, parts)
    }
}

impl From<&str> for RoutePath {
    fn from(value: &str) -> Self {
        Self {
            raw: value.to_string(),
            has_parts: value.contains(':'),
        }
    }
}

impl From<String> for PartsValue {
    fn from(value: String) -> Self {
        let mut map = BTreeMap::new();
        map.insert(0, value);
        PartsValue {
            pos: Some(map),
            name: None,
        }
    }
}

impl From<&str> for PartsValue {
    fn from(value: &str) -> Self {
        let mut map = BTreeMap::new();
        map.insert(0, value.to_string());
        PartsValue {
            pos: Some(map),
            name: None,
        }
    }
}

impl<K: ToString, V: ToString> From<HashMap<K, V>> for PartsValue {
    fn from(value: HashMap<K, V>) -> Self {
        Self {
            pos: None,
            name: Some(
                value
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v.to_string()))
                    .collect(),
            ),
        }
    }
}

impl<V: ToString> From<Vec<V>> for PartsValue {
    fn from(value: Vec<V>) -> Self {
        let mut map = BTreeMap::new();

        for (pos, value) in value.into_iter().enumerate() {
            map.insert(pos, value.to_string());
        }

        Self {
            pos: Some(map),
            name: None,
        }
    }
}

// TODO: Implement a macro to handle tuple with 7 values
impl<V: ToString> From<(V,)> for PartsValue {
    fn from(value: (V,)) -> Self {
        let mut map = BTreeMap::new();
        map.insert(0, value.0.to_string());

        Self {
            pos: Some(map),
            name: None,
        }
    }
}

// TODO: Remove when proc_macro is implemented
impl<V: ToString> From<(V, V)> for PartsValue {
    fn from(value: (V, V)) -> Self {
        let mut map = BTreeMap::new();
        map.insert(0, value.0.to_string());
        map.insert(1, value.1.to_string());

        Self {
            pos: Some(map),
            name: None,
        }
    }
}

// TODO: Remove when proc_macro is implemented
impl<V: ToString> From<(V, V, V)> for PartsValue {
    fn from(value: (V, V, V)) -> Self {
        let mut map = BTreeMap::new();
        map.insert(0, value.0.to_string());
        map.insert(1, value.1.to_string());
        map.insert(2, value.2.to_string());

        Self {
            pos: Some(map),
            name: None,
        }
    }
}

// TODO: Remove when proc_macro is implemented
impl<V: ToString> From<(V, V, V, V)> for PartsValue {
    fn from(value: (V, V, V, V)) -> Self {
        let mut map = BTreeMap::new();
        map.insert(0, value.0.to_string());
        map.insert(1, value.1.to_string());
        map.insert(2, value.2.to_string());
        map.insert(3, value.3.to_string());

        Self {
            pos: Some(map),
            name: None,
        }
    }
}
