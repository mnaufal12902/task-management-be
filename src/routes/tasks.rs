use actix_web::web::{self, route};
use crate::controllers::task;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/tasks")
            // Get Method
            .route("", web::get().to(task::get_all_task))
            .route("{id}", web::get().to(task::get_task))
            .route("{id}/status", web::get().to(task::get_task_status))
            
            // Post Method
            .route("", web::post().to(task::create_task))
            .route("/finished", web::post().to(task::create_finished_task))

            // Put Method
            .route("", web::put().to(task::update_task))

            // Delete Method
            .route("{id}", web::delete().to(task::delete_task))
    );
}
