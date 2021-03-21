use actix_web::{HttpResponse, web, HttpRequest};
use super::{
    models::{ProviderParam, Provider, ProviderInner},
    consul::implementation::providers,
    token::{
        get_bearer_token,
        get_claims,
    },
};
use reqwest::Method;
use std::ops::Not;
use super::proxy::proxy;

pub async fn ok() -> HttpResponse {
    HttpResponse::Ok().body("Ok")
}

pub async fn provider_health(request: HttpRequest, path: web::Path<ProviderParam>, body: web::Bytes) -> HttpResponse {
    let provider = match find_provider(&path.provider) {
        Ok(p) => p,
        Err(e) => return e
    };
    proxy(request, Method::GET, provider.url("/health"), body)
}

fn find_provider(name: &String) -> Result<ProviderInner, HttpResponse> {
    let o_provider = match providers() {
        Ok(p) => p,
        _ => return Err(HttpResponse::InternalServerError().body("Unable to get providers"))
    }.into_iter()
        .find(|p| &p.name == name);

    let provider = match o_provider {
        Some(p) => p,
        _ => return Err(HttpResponse::NotFound().body("No such provider found"))
    };
    Ok(provider)
}

pub async fn instances() -> HttpResponse {
    match providers() {
        Ok(v) => HttpResponse::Ok().json(v.into_iter().map(|p| p.ext()).collect::<Vec<Provider>>()),
        _ => HttpResponse::InternalServerError().body("Unable to get providers")
    }
}

pub async fn redirect_to_login(request: HttpRequest, param: web::Query<ProviderParam>, bytes: web::Bytes) -> HttpResponse {
    let provider = match find_provider(&param.provider) {
        Ok(p) => p,
        Err(e) => return e
    };
    if provider.oauth.not() {
        return HttpResponse::NotFound().body("It is not oauth provider");
    }
    proxy(request, Method::GET, provider.url("/auth/login"), bytes)
}

pub async fn get_token(request: HttpRequest, param: web::Query<ProviderParam>, bytes: web::Bytes) -> HttpResponse {
    let provider = match find_provider(&param.provider) {
        Ok(p) => p,
        Err(e) => return e
    };
    proxy(request, Method::POST, provider.url("/auth/token"), bytes)
}


pub async fn check(request: HttpRequest, bytes: web::Bytes) -> HttpResponse {
    let provider_name = match provider_from_auth_header(&request){
        Ok(name) => name,
        Err(e) => return e
    };
    let provider = match find_provider(&provider_name) {
        Ok(p) => p,
        Err(e) => return e
    };
    proxy(request, Method::POST, provider.url("/auth/check"), bytes)
}

pub async fn refresh(request: HttpRequest, bytes: web::Bytes) -> HttpResponse {
    let provider_name = match provider_from_auth_header(&request){
        Ok(name) => name,
        Err(e) => return e
    };
    let provider = match find_provider(&provider_name) {
        Ok(p) => p,
        Err(e) => return e
    };
    proxy(request, Method::POST, provider.url("/auth/refresh"), bytes)
}

fn provider_from_auth_header(request: &HttpRequest) -> Result<String, HttpResponse> {
    let auth = match request.headers().get("Authorization") {
        Some(h) => match h.to_str() {
            Ok(value) => value.to_string(),
            Err(_) => Err(HttpResponse::Unauthorized().body("No header auth value"))?
        },
        _ => Err(HttpResponse::Unauthorized().body("No auth token"))?
    };
    let token = match get_bearer_token(auth) {
        Some(token) => token,
        None => Err(HttpResponse::Unauthorized().body("Not a Bearer token"))?
    };
    get_claims(&token)
        .map_err(|_| HttpResponse::Unauthorized().body("Wrong claims"))
        .map(|c| c.oauth_provider)
}