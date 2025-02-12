use aide::OperationInput;
use axum::{extract::{FromRequest, Request}, http::StatusCode, Form, Json};
use serde::{de::DeserializeOwned, Serialize};
use validator::{Validate, ValidationErrors};

use crate::error::{HandlerError, HandlerResult};

/// Validator that validates the inner value.
///
/// This is using the `validator` crate to validate the
/// inner value. Used to validate the body of incoming requests.
pub struct Validated<T>(pub T);

/// A validation error response.
#[derive(Serialize)]
pub struct ValidationErrorResponse {
    #[serde(skip_serializing_if = "Vec::is_empty")]
	errors: Vec<ValidationErrorResponseItem>,
}

/// A single validation error item.
#[derive(Serialize)]
pub struct ValidationErrorResponseItem {
	field: String,
	problems: Vec<String>,
}

impl<S, T> FromRequest<S> for Validated<Json<T>>
where
	S: Send + Sync,
	T: DeserializeOwned + Validate,
{
	type Rejection = HandlerError<ValidationErrorResponse>;

	async fn from_request(req: Request, s: &S) -> Result<Self, Self::Rejection> {
		let json = Json::<T>::from_request(req, s).await.map_err(unparseable)?;
		validate(&json.0)?;
		Ok(Validated(json))
	}
}

impl<T> OperationInput for Validated<Json<T>> {}

impl<S, T> FromRequest<S> for Validated<Form<T>>
where
	S: Send + Sync,
	T: DeserializeOwned + Validate,
{
	type Rejection = HandlerError<ValidationErrorResponse>;

	async fn from_request(req: Request, s: &S) -> Result<Self, Self::Rejection> {
		let form = Form::<T>::from_request(req, s).await.map_err(unparseable)?;
		validate(&form.0)?;
		Ok(Validated(form))
	}
}

impl<T> OperationInput for Validated<Form<T>> {}

impl From<ValidationErrors> for ValidationErrorResponse {
	fn from(errors: ValidationErrors) -> Self {
		let display_errors = Vec::new();

		for (_field, _errors) in errors.field_errors() {
			todo!()
		}

		ValidationErrorResponse { errors: display_errors }
	}
}

#[inline]
fn validate<T: Validate>(data: T) -> HandlerResult<(), ValidationErrorResponse> {
    data.validate().map_err(|err| HandlerError::new(
		StatusCode::BAD_REQUEST,
		"Validation failed",
		"Was unable to validate you request."
	).with_extension(ValidationErrorResponse::from(err)))
}

#[inline]
fn unparseable<T: std::error::Error>(_: T) -> HandlerError<ValidationErrorResponse> {
    HandlerError::new(
        StatusCode::BAD_REQUEST,
        "Invalid request",
        "Couldn't parse request body."
    )
}
