use std::{
    collections::{BTreeMap, HashMap},
    fmt::Display,
    sync::Arc,
};

use axum::{
    body::Body,
    http::{response, Response},
    response::{sse::KeepAlive, Html, IntoResponse},
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
                named_map.insert(name.to_string(), map.get(&pos).unwrap().clone());
                // TODO: Handle when we get back none
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

    pub fn redirect_meta(&self) -> String {
        format!(
            "<meta http-equiv=\"Refresh\" content=\"0; URL={}\" />",
            &self.make_redirector(None).path()
        )
    }

    pub fn make_redirector(&self, parts: Option<HashMap<String, String>>) -> Redirector {
        Redirector::new(&self.raw, parts)
    }
}

impl From<&RoutePath> for Html<&'static str> {
    fn from(value: &RoutePath) -> Self {
        Html("for now")
    }
}

impl From<&RoutePath> for String {
    fn from(value: &RoutePath) -> Self {
        value.raw.clone()
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

// TODO: Implement a macro to handle tuple with 7 values
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
