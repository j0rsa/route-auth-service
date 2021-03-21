// #[macro_use]
extern crate log;

use std::env;
use actix_web::{App, HttpServer, web};
use env_logger;

mod providers;
use providers::handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let address = env::var("BIND_ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = env::var("BIND_PORT").unwrap_or("8080".to_string());

    HttpServer::new(move ||
        App::new()
            .service(web::resource("/health").route(web::get().to(handlers::ok)))
            .service(web::resource("/health/{provider}").route(web::get().to(handlers::provider_health)))
            .service(web::resource("/providers").route(web::get().to(handlers::instances)))
            .service(web::resource("/auth/login").route(web::get().to(handlers::redirect_to_login)))

            .service(web::resource("/auth/token").route(web::post().to(handlers::get_token)))
            .service(web::resource("/auth/check").route(web::get().to(handlers::check)))
            .service(web::resource("/auth/refresh").route(web::post().to(handlers::refresh)))
    )
        .bind(format!("{}:{}", &address, &port))?
        .run()
        .await
}
