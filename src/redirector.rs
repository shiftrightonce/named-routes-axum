use std::collections::HashMap;

use axum::{
    body::Body,
    http::{header, response::Builder, Response, StatusCode},
    response::{Html, IntoResponse},
};

#[derive(Debug, Clone)]
pub struct Redirector {
    parts: Option<HashMap<String, String>>,
    raw: String,
}

impl Redirector {
    pub(crate) fn new(raw: &str, parts: Option<HashMap<String, String>>) -> Self {
        Self {
            raw: raw.to_string(),
            parts,
        }
    }

    pub fn redirect<T: IntoResponse>(&self, response: T) -> Response<Body> {
        let mut response = response.into_response();

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
        if self.parts.is_some() {
            for (k, v) in self.parts.as_ref().unwrap().iter() {
                raw = raw.replace(k, v);
            }
        }

        raw
    }
}
