use actix_web::web;

pub mod user;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg
        .service(user::list_users)
        .service(user::create_user_handler);
}