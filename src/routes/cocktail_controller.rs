use actix_web::{get, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::services::cocktail_service::{get_all_cocktails};

#[get("/cocktails")]
async fn get_all_users_handler(pool: web::Data<PgPool>) -> impl Responder {
    match get_all_cocktails(pool.get_ref()).await {
        Ok(cocktails) => HttpResponse::Ok().json(cocktails),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}