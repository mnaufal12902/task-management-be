use actix_web::web;
use crate::controllers::auth;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
        // Get Method
        
        //Post Method
        .route("/login", web::post().to(auth::login_handler))
        .route("/logout", web::post().to(auth::logout_handler))

        // Delete Method
    );
}
