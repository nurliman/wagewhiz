use async_graphql::ErrorExtensions;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("User is not authenticated")]
    Unauthenticated,

    #[error("User does not have permission to perform this action")]
    Forbidden,

    #[error("Invalid user input: {0}")]
    BadInput(String),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Internal server error")]
    InternalError,
}

impl ErrorExtensions for AppError {
    fn extend(&self) -> async_graphql::Error {
        self.extend_with(|err, e| match err {
            AppError::Unauthenticated => e.set("code", "UNAUTHENTICATED"),
            AppError::Forbidden => e.set("code", "FORBIDDEN"),
            AppError::BadInput(_) => e.set("code", "BAD_USER_INPUT"),
            AppError::ResourceNotFound(_) => e.set("code", "RESOURCE_NOT_FOUND"),
            AppError::ValidationError(_) => e.set("code", "VALIDATION_ERROR"),
            AppError::InternalError => e.set("code", "INTERNAL_SERVER_ERROR"),
        })
    }
}
