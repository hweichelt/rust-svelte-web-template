use axum::extract::{FromRequest, Request, rejection::JsonRejection};

use crate::error::AppError;

/// JSON body extractor that reports malformed input as a regular
/// `validation_error` instead of axum's plain-text rejection.
pub struct Json<T>(pub T);

impl<S, T> FromRequest<S> for Json<T>
where
    axum::Json<T>: FromRequest<S, Rejection = JsonRejection>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let axum::Json(value) = axum::Json::<T>::from_request(req, state)
            .await
            .map_err(|err| AppError::Validation(err.body_text()))?;
        Ok(Self(value))
    }
}
