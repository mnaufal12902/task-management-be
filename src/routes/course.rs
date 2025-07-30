use actix_web::web;
use crate::controllers::course;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/course")
        // Get Method
        .route("", web::get().to(course::get_all_subject))

        // Post Method
        .route("", web::post().to(course::create_subject))
    );
}
