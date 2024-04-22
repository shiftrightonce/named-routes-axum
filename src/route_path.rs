use std::collections::HashMap;

use axum::{
    body::Body,
    http::{header, response::Builder, status, Response, StatusCode},
    response::{Html, IntoResponse},
};

#[derive(Debug, Default, Clone)]
pub struct Validator;

#[derive(Debug, Default, Clone)]
pub struct RoutePath {
    raw: String,
    pieces: Option<HashMap<String, Validator>>,
}

impl RoutePath {
    pub fn redirect<T: IntoResponse>(&self, raw: T) -> Response<Body> {
        let mut response = raw.into_response();

        response.headers_mut().append(
            header::LOCATION,
            header::HeaderValue::from_str(&self.raw).unwrap(),
        );
        *response.status_mut() = StatusCode::TEMPORARY_REDIRECT;
        response
    }

    pub fn redirect_meta(&self) -> String {
        format!(
            "<meta http-equiv=\"Refresh\" content=\"0; URL={}\" />",
            &self.raw
        )
    }

    pub fn redirect_t<T>(&self, body: T) -> Response<T> {
        Builder::new()
            .header(
                header::LOCATION,
                header::HeaderValue::from_str(self.generate_clean_path().as_str()).unwrap(),
            )
            .status(StatusCode::TEMPORARY_REDIRECT)
            .body(body)
            .unwrap()
    }

    fn generate_clean_path(&self) -> String {
        // TODO: Fill the path variables
        self.raw.clone()
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
            pieces: None,
        }
    }
}
