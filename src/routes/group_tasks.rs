use crate::controllers::group_task;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/group-task")
            // Get Method
            .route("{id}", web::get().to(group_task::get_group_tasks))
            // Post Method
            .route("", web::post().to(group_task::add_finished_group_task))
            // Delete Method
            .route("", web::delete().to(group_task::remove_finished_group_task))
    );
}
