use actix_web::web;

pub mod user;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg
        .service(user::get_all_users_handler)
        .service(user::create_user_handler);
}