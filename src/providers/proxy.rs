use actix_web::{
    web,
    web::Buf,
    HttpRequest,
    HttpResponse,
    http::{
        StatusCode,
    },
};
use reqwest::{
    ClientBuilder,
    RedirectPolicy,
    Url,
    Method,
    header::{
        HeaderMap,
        HeaderName,
        HeaderValue,
    },
};

use std::str::FromStr;


pub fn proxy(request: HttpRequest, method: reqwest::Method, target_url: Url, body: web::Bytes) -> HttpResponse {
    let result = proxy_request(&request, method, target_url, body);
    match result {
        Ok(r) => proxy_response(r),
        _ => HttpResponse::InternalServerError().body("Unable to process request")
    }
}

fn proxy_request(request: &HttpRequest, method: Method, target_url: Url, body: web::Bytes) -> Result<reqwest::Response, reqwest::Error> {
    let client = ClientBuilder::new()
        .redirect(RedirectPolicy::none())
        .build().unwrap();
    let body_bytes = body.bytes().to_vec();
    let result = client.request(method, target_url)
        .headers(request_headers(&request))
        .body(body_bytes)
        .send();
    result
}

fn proxy_response(mut r: reqwest::Response) -> HttpResponse {
    let builder = HttpResponse::build(
        StatusCode::from_u16(r.status().as_u16()).unwrap());
    with_response_headers(builder, r.headers())
        .body(r.text().unwrap_or_default())
}

fn with_response_headers(mut builder: actix_web::dev::HttpResponseBuilder, headers: &HeaderMap) -> actix_web::dev::HttpResponseBuilder {
    for (k, v) in headers {
        builder.header(actix_web::http::HeaderName::from_str(&k.to_string()).unwrap(), v.as_bytes());
    }
    builder
}

fn request_headers(reqwest: &HttpRequest) -> reqwest::header::HeaderMap {
    let headers: Vec<_> = reqwest.headers()
        .iter()
        .map(|item| (item.0.to_string(), item.1.as_bytes()))
        .collect();
    let mut map = reqwest::header::HeaderMap::new();
    for (k,v) in headers {
        map.insert(HeaderName::from_str(&k).unwrap(), HeaderValue::from_bytes(v).unwrap());
    }
    map
}