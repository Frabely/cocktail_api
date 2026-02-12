use actix_web::{get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::common::dtos::create_user_dto::CreateUserDto;
use crate::services::user_service::{create_user, get_all_users};

#[get("/users")]
async fn get_all_users_handler(pool: web::Data<PgPool>) -> impl Responder {
    match get_all_users(pool.get_ref()).await {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

/*
#[post("/user")]
async fn create_user_handler(
    pool: web::Data<PgPool>,
    user_input: web::Json<CreateUserDto>,
) -> impl Responder {
    match create_user(pool.get_ref(), user_input.into_inner()).await {
        Ok(user_output) => HttpResponse::Ok().json(user_output),
        Err(sqlx::Error::RowNotFound) => HttpResponse::BadRequest().body("User could not be created"),
        Err(err) => {
            log::error!("Failed to create user: {:?}", err);
            HttpResponse::InternalServerError().body("Error creating user")
        }
    }
}*/