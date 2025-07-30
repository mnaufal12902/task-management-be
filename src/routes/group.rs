use actix_web::web;
use crate::controllers::group;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/groups")
        // Get Method
        .route("", web::get().to(group::get_all_groups))
        
        //Post Method
        .route("", web::post().to(group::create_grup))
        .route("{id}/members", web::post().to(group::add_member_to_group))

        // Delete Method
        .route("{id}", web::delete().to(group::delete_group))
        .route("{id}/members", web::delete().to(group::remove_member_from_group))
    );
}
