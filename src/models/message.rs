use std::fmt::{self, Display};

#[derive(Debug)]
pub enum ErrorMessage {
    Authorized,
    CantBeNull,
    CreateDataSuccess,
    DeleteSuccess,
    Duplicate,
    InsufficientPermissions,
    InvalidAuthHeader,
    InvalidAuthScheme,
    LoginInvalid,
    LoginSuccess,
    LogoutSuccess,
    NoAuthHeader,
    NotFound,
    RefreshTokenInvalid,
    Success,
    TokenInvalid,
    UnAuthorized,
    UpdateDataSuccess,
    UserAlreadyInGroup,
    UserNotMatch,

    // 🔽 New basic error types
    DataTooLong,
    CheckConstraintFailed,
    ForeignKeyViolation,

    // 🔽 Error with details
    DatabaseError { details: String },
    Error { details: String },
    FailedAddMember { details: String },
    FailedFetchFinishedTask { details: String },
    FailedFetchUnFinishedTask { details: String },
    TaskTypeError { details: String },
    TokenDecodeError { details: String },
    TokenGenerateFailed { details: String },
    UnhanledErrorCode { code: String, details: String },
}

impl Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // 🔁 Basic messages
            ErrorMessage::Authorized => write!(f, "Authorized"),
            ErrorMessage::CantBeNull => write!(f, "Can't be null"),
            ErrorMessage::CreateDataSuccess => write!(f, "Create data success"),
            ErrorMessage::DeleteSuccess => write!(f, "Delete data success"),
            ErrorMessage::Duplicate => write!(f, "Data duplicated"),
            ErrorMessage::InsufficientPermissions => {
                write!(f, "Insufficient permissions for this action")
            }
            ErrorMessage::InvalidAuthHeader => write!(f, "Invalid authorization header"),
            ErrorMessage::InvalidAuthScheme => write!(f, "Invalid authorization scheme"),
            ErrorMessage::LoginInvalid => write!(f, "Username or password is wrong"),
            ErrorMessage::LoginSuccess => write!(f, "Login successful"),
            ErrorMessage::LogoutSuccess => write!(f, "Logout successful"),
            ErrorMessage::NoAuthHeader => write!(f, "No authorization header provided"),
            ErrorMessage::NotFound => write!(f, "Data not found"),
            ErrorMessage::RefreshTokenInvalid => write!(f, "Refresh token invalid"),
            ErrorMessage::Success => write!(f, "Success"),
            ErrorMessage::TokenInvalid => write!(f, "Token invalid"),
            ErrorMessage::UnAuthorized => write!(f, "Unauthorized"),
            ErrorMessage::UpdateDataSuccess => write!(f, "Update data successfully"),
            ErrorMessage::UserAlreadyInGroup => write!(f, "Some user already in group"),
            ErrorMessage::UserNotMatch => write!(f, "User not match"),

            // ✅ Newly added
            ErrorMessage::DataTooLong => write!(f, "Data too long for column"),
            ErrorMessage::CheckConstraintFailed => write!(f, "Check constraint failed"),
            ErrorMessage::ForeignKeyViolation => write!(f, "Foreign key constraint violated"),

            // 🧩 With details
            ErrorMessage::DatabaseError { details } => write!(f, "Database error: {}", details),
            ErrorMessage::Error { details } => write!(f, "Error: {}", details),
            ErrorMessage::FailedAddMember { details } => {
                write!(f, "Failed to add member: {}", details)
            }
            ErrorMessage::FailedFetchFinishedTask { details } => {
                write!(f, "Failed to fetch finished task: {}", details)
            }
            ErrorMessage::FailedFetchUnFinishedTask { details } => {
                write!(f, "Failed to fetch unfinished task: {}", details)
            }
            ErrorMessage::TaskTypeError { details } => {
                write!(f, "Task type error: {}", details)
            }
            ErrorMessage::TokenDecodeError { details } => {
                write!(f, "Token decode error: {}", details)
            }
            ErrorMessage::TokenGenerateFailed { details } => {
                write!(f, "Token generation failed: {}", details)
            }
            ErrorMessage::UnhanledErrorCode { code, details } => {
                write!(f, "Unhandled error code {}: {}", code, details)
            }
        }
    }
}
