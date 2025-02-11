use actix_web::{App, HttpServer};
mod db;
mod routes;
mod models;
mod services;
mod common;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::init_db().await.expect("Datenbankverbindung fehlgeschlagen");

    HttpServer::new(move || {
        App::new()
            .app_data(pool.clone())
            .configure(routes::init)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
