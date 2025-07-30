use actix_web::web;
use crate::controllers::session;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/session")
        // Post Method
        .route("/refresh-token", web::post().to(session::renew_access_token))
    );
}
