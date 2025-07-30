use actix_web::web;
use crate::controllers::user;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            // Get Method
            .route("", web::get().to(user::get_all_users))
            .route("{username}", web::get().to(user::get_user))

            // Post Method
            .route("", web::post().to(user::create_user))

            // Put Method
            .route("", web::put().to(user::update_data_user))
            // .route("{username}/upload-profile-picture", web::post().to(user::upload_profile_picture))
            
            // Delete Method
            .route("{username}", web::delete().to(user::delete_user))
    );
}
