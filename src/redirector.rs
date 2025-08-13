use std::collections::HashMap;

use axum::{
    body::Body,
    http::{header, response::Builder, Response, StatusCode},
    response::IntoResponse,
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

    /// Returns a reponse instance that will found/302 redirects client
    pub fn redirect<T: IntoResponse>(&self, response: T) -> Response<Body> {
        let mut response = response.into_response();

        response.headers_mut().append(
            header::LOCATION,
            header::HeaderValue::from_str(self.path().as_str()).unwrap(),
        );
        *response.status_mut() = StatusCode::FOUND;
        response
    }

    pub fn empty_redirect(&self) -> Response<Body> {
        self.redirect(())
    }

    /// Returns an html meta tag with a refresh
    ///
    /// ```html
    ///  <meta http-equiv="Refresh" content="0; URL=http::/foo.com" />,
    /// ```
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
            .status(StatusCode::FOUND)
            .body(body)
            .unwrap()
    }

    /// Returns the build route's path
    pub fn path(&self) -> String {
        let mut raw = self.raw.clone();
        if self.parts.is_some() {
            for (k, v) in self.parts.as_ref().unwrap().iter() {
                let name = if k.starts_with('{') && k.ends_with('}') {
                    k
                } else {
                    &format!("{{{}}}", k)
                };
                raw = raw.replace(name, v);
            }
        }

        raw
    }
}

#[cfg(test)]
mod test {
    use super::Redirector;
    use std::collections::HashMap;

    #[test]
    fn test_path_with_no_parts() {
        let mut parts = HashMap::new();
        parts.insert("{yahoo}".to_string(), "google".to_string());

        let redirector = Redirector::new("/yahoo", Some(parts));
        assert_eq!(redirector.path().as_str(), "/yahoo");
    }

    #[test]
    fn test_path_with_a_part() {
        let mut parts = HashMap::new();
        parts.insert("yahoo".to_string(), "google".to_string());

        let redirector = Redirector::new("/{yahoo}", Some(parts));
        assert_eq!(redirector.path().as_str(), "/google");
    }

    #[test]
    fn test_path_with_parts() {
        let mut parts = HashMap::new();
        parts.insert("user_id".to_string(), "1234".to_string());
        parts.insert("product_id".to_string(), "4567".to_string());

        let redirector = Redirector::new("/user/{user_id}/product/{product_id}", Some(parts));
        assert_eq!(redirector.path().as_str(), "/user/1234/product/4567");
    }

    #[test]
    fn test_path_with_duplicate_parts() {
        let mut parts = HashMap::new();
        parts.insert("id".to_string(), "1234".to_string());

        let redirector = Redirector::new("/user/{id}/{id}", Some(parts));
        assert_eq!(redirector.path().as_str(), "/user/1234/1234");
    }
}
