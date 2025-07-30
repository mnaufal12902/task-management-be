use crate::controllers::user_task;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user-task")
            // Get Method
            .route("{id}", web::get().to(user_task::get_user_tasks))
            // Post Method
            .route("", web::post().to(user_task::add_finished_user_task))
            // Delete Method
            .route("", web::delete().to(user_task::remove_finished_user_task))
    );
}
