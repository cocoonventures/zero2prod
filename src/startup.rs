#![allow(unused_imports)]
use crate::routes::{health_check, subscribe, subscriptions};
use actix_web::dev::Server;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use sea_orm::{ConnectOptions, DatabaseConnection};
use std::net::TcpListener;

pub fn run(listener: TcpListener, db_pool: ConnectOptions) -> Result<Server, std::io::Error> {
    // wrap db connection in a smart pointer
    let db_pool = web::Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
