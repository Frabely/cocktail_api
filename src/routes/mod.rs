use actix_web::web;

pub mod user;
pub mod cocktail_controller;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg
        // .service(user::get_all_users_handler)
        // .service(user::create_user_handler)
        .service(cocktail_controller::get_all_users_handler);
}