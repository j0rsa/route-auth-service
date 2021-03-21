use std::env;

pub fn consul_url() -> String {
    env::var("CONSUL_URL").unwrap_or("http://localhost:8500".to_string())
}

pub fn consul_service() -> String {
    env::var("CONSUL_SERVICE").unwrap_or("auth".to_string())
}
