use actix_web::{web, App, HttpServer};
use env_logger;
// use crate::services::cocktail_service::get_all_cocktails;

mod db;
mod routes;
mod models;
mod services;
mod common;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let pool = db::init_db().await.expect("Datenbankverbindung felgeschlagen");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(routes::init)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await


}
