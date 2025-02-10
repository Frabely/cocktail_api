use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::services::user_service::get_all_users;

#[get("/users")]
async fn list_users(pool: web::Data<PgPool>) -> impl Responder {
    match get_all_users(pool.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users), // Gibt die Benutzer als JSON zurück
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}