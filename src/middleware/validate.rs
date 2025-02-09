use aide::OperationInput;
use axum::{extract::{FromRequest, Request}, http::StatusCode, Json};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::HandlerError;

/// Wrapper around `axum::extract::Json` that validates the inner value.
///
/// This is using the `validator` crate to validate the inner value.
/// If this is successful, the inner value is returned as a `ValidatedJson`.
pub struct Validated<T>(pub T);

impl<S, T> FromRequest<S> for Validated<Json<T>>
where
	S: Send + Sync,
	T: DeserializeOwned + Validate,
{
	type Rejection = HandlerError;

	async fn from_request(req: Request, s: &S) -> Result<Self, Self::Rejection> {
		let json = axum::extract::Json::<T>::from_request(req, s)
			.await
			.map_err(|_| HandlerError::new(
				StatusCode::BAD_REQUEST,
				"Invalid request",
				"Couldn't parse your request."
			))?;

		json.0.validate().map_err(|_| HandlerError::new(
			StatusCode::BAD_REQUEST,
			"Validation failed",
			"Was unable to validate you request."
		))?;

		Ok(Validated(json))
	}
}

impl<T> OperationInput for Validated<Json<T>> {}
