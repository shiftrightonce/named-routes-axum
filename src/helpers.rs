use axum::{body::Body, http::Response, response::IntoResponse};

use crate::{NamedRoutesService, PartsValue};

pub fn redirect_with<V: Into<PartsValue>>(parts: V, route_name: &str) -> impl IntoResponse {
    NamedRoutesService::new()
        .get(route_name)
        .unwrap()
        .with(parts)
        .redirect("")
}

pub fn try_redirect_with<V: Into<PartsValue>>(
    parts: V,
    route_name: &str,
) -> Option<Response<Body>> {
    NamedRoutesService::new()
        .get(route_name)
        .map(|route| route.with(parts).redirect(""))
}

pub fn redirect(route_name: &str) -> impl IntoResponse {
    NamedRoutesService::new()
        .get(route_name)
        .unwrap()
        .redirect("")
}

pub fn try_redirect(route_name: &str) -> Option<Response<Body>> {
    NamedRoutesService::new()
        .get(route_name)
        .map(|route| route.redirect(""))
}

pub fn has_route(route_name: &str) -> bool {
    NamedRoutesService::new().has(route_name)
}

pub fn has_parts(route_name: &str) -> bool {
    let service = NamedRoutesService::new();
    service.has(route_name) && service.get(route_name).unwrap().has_parts()
}

pub fn get_path(route_name: &str) -> String {
    NamedRoutesService::new().get_path(route_name).unwrap()
}

pub fn try_get_path(route_name: &str) -> Option<String> {
    NamedRoutesService::new().get_path(route_name)
}

pub fn get_path_with<V: Into<PartsValue>>(route_name: &str, parts: V) -> String {
    NamedRoutesService::new()
        .get_path_with(route_name, parts)
        .unwrap()
}

pub fn try_get_path_with<V: Into<PartsValue>>(route_name: &str, parts: V) -> Option<String> {
    NamedRoutesService::new().get_path_with(route_name, parts)
}
