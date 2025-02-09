use aide::OperationInput;
use axum::{extract::{FromRequest, Request}, http::StatusCode, Form, Json};
use serde::{de::DeserializeOwned, Serialize};
use validator::{Validate, ValidationErrors};

use crate::error::HandlerError;

/// Wrapper around `axum::extract::Json` that validates the inner value.
///
/// This is using the `validator` crate to validate the inner value.
/// If this is successful, the inner value is returned as a `ValidatedJson`.
pub struct Validated<T>(pub T);

#[derive(Serialize)]
pub struct ValidationError {
	errors: Vec<ValidationErrorItem>,
}

#[derive(Serialize)]
pub struct ValidationErrorItem {
	field: String,
	message: String,
}

impl<S, T> FromRequest<S> for Validated<Json<T>>
where
	S: Send + Sync,
	T: DeserializeOwned + Validate,
{
	type Rejection = HandlerError<ValidationError>;

	async fn from_request(req: Request, s: &S) -> Result<Self, Self::Rejection> {
		let json = Json::<T>::from_request(req, s)
			.await
			.map_err(|_| HandlerError::new(
				StatusCode::BAD_REQUEST,
				"Invalid request",
				"Couldn't parse your request."
			))?;

		json.0.validate().map_err(|err| HandlerError::new(
			StatusCode::BAD_REQUEST,
			"Validation failed",
			"Was unable to validate you request."
		).with_extension(ValidationError::from(err)))?;

		Ok(Validated(json))
	}
}

impl<T> OperationInput for Validated<Json<T>> {}

impl<S, T> FromRequest<S> for Validated<Form<T>>
where
	S: Send + Sync,
	T: DeserializeOwned + Validate,
{
	type Rejection = HandlerError<ValidationError>;

	async fn from_request(req: Request, s: &S) -> Result<Self, Self::Rejection> {
		let form = Form::<T>::from_request(req, s)
			.await
			.map_err(|_| HandlerError::new(
				StatusCode::BAD_REQUEST,
				"Invalid request",
				"Couldn't parse your request."
			))?;

		form.0.validate().map_err(|err| HandlerError::new(
			StatusCode::BAD_REQUEST,
			"Validation failed",
			"Was unable to validate you request."
		).with_extension(ValidationError::from(err)))?;

		Ok(Validated(form))
	}
}

impl<T> OperationInput for Validated<Form<T>> {}

impl From<ValidationErrors> for ValidationError {
	fn from(errors: ValidationErrors) -> Self {
		let mut items = Vec::new();

		for (field, errors) in errors.field_errors() {
			for error in errors {
				items.push(ValidationErrorItem {
					field: field.to_string(),
					message: error.to_string(),
				});
			}
		}

		ValidationError { errors: items }
	}
}
