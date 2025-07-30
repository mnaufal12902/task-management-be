use std::borrow::Cow;

use actix_web::{HttpResponse, cookie::Cookie};
use serde::{Deserialize, Serialize};
use sqlx::Error;

use crate::models::{message::ErrorMessage, status::Status};

#[derive(Serialize, Deserialize)]
pub struct ApiResponder<T> {
    pub status: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponder<T> {
    pub fn success(message: String, data: Option<T>) -> HttpResponse
    where
        T: Serialize,
    {
        HttpResponse::Ok().json(ApiResponder {
            status: Status::Success.into(),
            message,
            data,
        })
    }

    pub fn created(message: String, data: Option<T>) -> HttpResponse
    where
        T: Serialize,
    {
        HttpResponse::Created().json(ApiResponder {
            status: Status::Created.into(),
            message,
            data,
        })
    }

    pub fn error(message: String, data: Option<T>) -> HttpResponse
    where
        T: Serialize,
    {
        HttpResponse::InternalServerError().json(ApiResponder {
            status: Status::InternalServerError.into(),
            message,
            data,
        })
    }

    pub fn unauthorized(message: String, data: Option<T>) -> HttpResponse
    where
        T: Serialize,
    {
        HttpResponse::Unauthorized().json(ApiResponder {
            status: Status::UnAuthorized.into(),
            message,
            data,
        })
    }

    pub fn conflict(message: String, data: Option<T>) -> HttpResponse
    where
        T: Serialize,
    {
        HttpResponse::Conflict().json(ApiResponder {
            status: Status::Conflict.into(),
            message,
            data,
        })
    }

    pub fn unprocessable_entity(message: String, data: Option<T>) -> HttpResponse
    where
        T: Serialize,
    {
        HttpResponse::UnprocessableEntity().json(ApiResponder {
            status: Status::Conflict.into(),
            message,
            data,
        })
    }

    pub fn not_found(message: String, data: Option<T>) -> HttpResponse
    where
        T: Serialize,
    {
        HttpResponse::NotFound().json(ApiResponder {
            status: Status::NotFound.into(),
            message,
            data,
        })
    }

    pub fn bad_request(message: String, data: Option<T>) -> HttpResponse
    where
        T: Serialize,
    {
        HttpResponse::BadRequest().json(ApiResponder {
            status: Status::BadRequest.into(),
            message,
            data,
        })
    }

    pub fn success_with_cookie(
        message: String,
        data: Option<T>,
        cookies: Vec<Cookie<'static>>,
    ) -> HttpResponse
    where
        T: Serialize,
    {
        let mut builder = HttpResponse::Ok();

        for cookie in cookies {
            builder.cookie(cookie);
        }

        builder.json(ApiResponder {
            status: Status::Success.into(),
            message,
            data,
        })
    }

    pub fn handle_error(error: Error) -> HttpResponse {
        match error {
            // 🔴 SQL-level database errors (e.g., constraint violations)
            Error::Database(db_err) => match db_err.code() {
                Some(Cow::Borrowed("1062") | Cow::Borrowed("23000")) => {
                    // Duplicate entry (UNIQUE or PRIMARY KEY constraint)
                    ApiResponder::conflict(ErrorMessage::Duplicate.to_string(), None::<()>)
                }
                Some(Cow::Borrowed("1048")) => {
                    // NULL value in a NOT NULL column
                    ApiResponder::bad_request(ErrorMessage::CantBeNull.to_string(), None::<()>)
                }
                Some(Cow::Borrowed("1452")) | Some(Cow::Borrowed("1451")) => {
                    // Foreign key constraint violation (insert/update/delete)
                    ApiResponder::unprocessable_entity(
                        ErrorMessage::ForeignKeyViolation.to_string(),
                        None::<()>,
                    )
                }
                Some(Cow::Borrowed("1406")) => {
                    // Data too long for column (VARCHAR overflow)
                    ApiResponder::bad_request(ErrorMessage::DataTooLong.to_string(), None::<()>)
                }
                Some(Cow::Borrowed("3819")) => {
                    // Check constraint failed (MySQL 8+)
                    ApiResponder::bad_request(
                        ErrorMessage::CheckConstraintFailed.to_string(),
                        None::<()>,
                    )
                }
                Some(code) => {
                    // Other known database error with code
                    ApiResponder::error(
                        ErrorMessage::UnhanledErrorCode {
                            code: code.to_string(),
                            details: db_err.message().to_string(),
                        }
                        .to_string(),
                        None::<()>,
                    )
                }
                None => {
                    // Unknown database error (no error code provided)
                    ApiResponder::error(
                        ErrorMessage::DatabaseError {
                            details: db_err.message().to_string(),
                        }
                        .to_string(),
                        None::<()>,
                    )
                }
            },

            // 🔧 SQLx configuration error (misconfigured database connection, etc.)
            Error::Configuration(err) => {
                ApiResponder::error(format!("Configuration error: {}", err), None::<()>)
            }

            // 💻 I/O error (network, file, etc.)
            Error::Io(err) => ApiResponder::error(format!("I/O error: {}", err), None::<()>),

            // 🔐 TLS/SSL error
            Error::Tls(err) => ApiResponder::error(format!("TLS error: {}", err), None::<()>),

            // 🔁 Protocol-level error from SQL driver
            Error::Protocol(err) => {
                ApiResponder::error(format!("Protocol error: {}", err), None::<()>)
            }

            // 🔍 Expected row not found (e.g., fetch_one failed)
            Error::RowNotFound => ApiResponder::not_found("Data not found".to_string(), None::<()>),

            // ⚠️ Rust type not found (usually incorrect mapping)
            Error::TypeNotFound { type_name } => {
                ApiResponder::error(format!("Type not found: {}", type_name), None::<()>)
            }

            // 🧮 Column index is out of bounds in result row
            Error::ColumnIndexOutOfBounds { index, len } => ApiResponder::error(
                format!(
                    "Column index {} out of bounds (row has {} columns)",
                    index, len
                ),
                None::<()>,
            ),

            // 🔎 Column name does not exist in query result
            Error::ColumnNotFound(name) => {
                ApiResponder::error(format!("Column not found: {}", name), None::<()>)
            }

            // 🧨 Error decoding a column to Rust type
            Error::ColumnDecode { index, source } => ApiResponder::error(
                format!("Column decode error at index {}: {}", index, source),
                None::<()>,
            ),

            // 💥 General decode error (e.g., struct FromRow implementation failed)
            Error::Decode(err) => ApiResponder::error(format!("Decode error: {}", err), None::<()>),

            // ⌛ Connection pool timeout
            Error::PoolTimedOut => {
                ApiResponder::error("Database connection timed out".to_string(), None::<()>)
            }

            // ❌ Connection pool already closed
            Error::PoolClosed => ApiResponder::error(
                "Database connection pool has been closed".to_string(),
                None::<()>,
            ),

            // 🧯 Worker thread crashed (rare)
            Error::WorkerCrashed => {
                ApiResponder::error("Database worker thread crashed".to_string(), None::<()>)
            }

            // 🔄 SQLx migration error (e.g., apply, validate)
            Error::Migrate(err) => {
                ApiResponder::error(format!("Migration error: {}", err), None::<()>)
            }

            // 🧩 Catch-all fallback for any unexpected SQLx error
            other => ApiResponder::error(format!("Unhandled SQLx error: {:?}", other), None::<()>),
        }
    }
}
