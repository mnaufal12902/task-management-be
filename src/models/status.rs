use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Status {
    Success = 200,
    Created = 201,
    BadRequest = 400,
    UnAuthorized = 401,
    NotFound = 404,
    Conflict = 409,
    UnprocessableEntity = 422,
    InternalServerError = 500
}

impl From<Status> for i32 {
    fn from(status: Status) -> Self {
        match status {
            Status::Success => 200,
            Status::Created => 201,
            Status::BadRequest => 400,
            Status::UnAuthorized => 401,
            Status::NotFound => 404,
            Status::Conflict => 409,
            Status::UnprocessableEntity => 422,
            Status::InternalServerError => 500,
        }
    }
}