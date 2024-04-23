use std::collections::HashMap;

use axum::{
    body::Body,
    http::{header, response::Builder, status, Response, StatusCode},
    response::{Html, IntoResponse},
};

#[derive(Debug, Default, Clone)]
pub struct Validator(String);

#[derive(Debug, Default, Clone)]
pub struct RoutePath {
    raw: String,
    pieces: Option<HashMap<String, Validator>>,
}

impl RoutePath {
    pub fn set_param(&mut self, key: &str, value: impl ToString) -> &mut Self {
        if self.pieces.is_some() {
            self.pieces
                .as_mut()
                .unwrap()
                .insert(format!(":{}", key), value.into());
        }

        self
    }

    pub fn redirect<T: IntoResponse>(&self, raw: T) -> Response<Body> {
        let mut response = raw.into_response();

        response.headers_mut().append(
            header::LOCATION,
            header::HeaderValue::from_str(self.path().as_str()).unwrap(),
        );
        *response.status_mut() = StatusCode::TEMPORARY_REDIRECT;
        response
    }

    pub fn redirect_meta(&self) -> String {
        format!(
            "<meta http-equiv=\"Refresh\" content=\"0; URL={}\" />",
            &self.path()
        )
    }

    pub fn redirect_t<T>(&self, body: T) -> Response<T> {
        Builder::new()
            .header(
                header::LOCATION,
                header::HeaderValue::from_str(self.path().as_str()).unwrap(),
            )
            .status(StatusCode::TEMPORARY_REDIRECT)
            .body(body)
            .unwrap()
    }

    pub fn path(&self) -> String {
        let mut raw = self.raw.clone();
        if self.pieces.is_some() {
            for (k, v) in self.pieces.as_ref().unwrap().iter() {
                raw = raw.replace(k, v.0.as_str());
            }
        }

        raw
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
        if value.contains(':') {
            let pieces = value
                .split('/')
                .into_iter()
                .filter(|entry| entry.contains(':'))
                .map(|entry| (entry.to_string(), Validator::default()))
                .collect();

            Self {
                raw: value.to_string(),
                pieces: Some(pieces),
            }
        } else {
            Self {
                raw: value.to_string(),
                pieces: None,
            }
        }
    }
}

impl<T: ToString> From<T> for Validator {
    fn from(value: T) -> Self {
        Self(value.to_string())
    }
}
