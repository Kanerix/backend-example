//! Error module for endpoint handlers.
//!
//! This module will follow the [Problem Details for HTTP APIs](https://datatracker.ietf.org/doc/html/rfc9457) specification.

use aide::OperationOutput;
use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
	Json,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A type alias for [`Result<T, HandlerError>`].
///
/// Used by handlers to return a response or an structured error.
pub type HandlerResult<T, D = ()> = std::result::Result<T, HandlerError<D>>;

/// Represents an error returned by a handler.
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
pub struct HandlerError<D = ()>
where
	D: Serialize + Send + Sync,
{
	/// HTTP status code for the error.
	#[serde(skip)]
	status: StatusCode,
	/// The error title.
	///
	/// Short and precise text that gives an indication
	/// of what the error is about.
	title: String,
	/// The detailed error message.
	///
	/// A more detailed description of what wen't wrong
	/// or what the next step is.
	detail: String,
	/// The instance of the error.
	///
	/// Does not get send to the client if it's [`None`].
	/// This is a unique identifier for the error. This will usually
	/// be the endpoint that the error occurred in.
	#[serde(skip_serializing_if = "Option::is_none")]
	instance: Option<String>,
	/// Additional information about the error.
	///
	/// Does not get send to the client if it's [`None`].
	/// The [`Some`] variant should implement [`Serialize`] so that
	/// an OpenAPI schema can be generated for the type.
	#[serde(skip_serializing_if = "Option::is_none", flatten)]
	extension: Option<D>,
	/// The log ID of the error.
	///
	/// This is automatically set when the response contains an error
	/// that should be tracked. This is not public, so that it is never
	/// set manually, since that might break how you identify the error.
	///
	/// This field is sent to the client instead of the acctual error
	/// that occured. This is way more secure, since the acctual error might
	/// contain information that should not be leaked and might help attackers
	/// understand how to exploit the application.
	#[serde(skip_serializing_if = "Option::is_none")]
	log_id: Option<String>,
	/// The actual error that occurred.
	///
	/// There might no be an actual error, in which case this
	/// field is [`None`]. Should never be exposed to the client
	/// for security reasons. This is why we skip Serilization.
	///
	/// If this field contains an error, the log_id field should
	/// also be present, to identify the error in the logs.
	#[serde(skip)]
	inner: Option<anyhow::Error>,
}

impl<D> HandlerError<D>
where
	D: Serialize + Send + Sync,
{
	/// Create a new [`HandlerError`] with status code, header and message.
	///
	/// All optional fields are `None` by default. These can be set using functions
	/// found on the struct.
	pub fn new(
		status_code: StatusCode,
		header: impl Into<String>,
		message: impl Into<String>,
	) -> Self {
		Self {
			status: status_code,
			title: header.into(),
			detail: message.into(),
			instance: None,
			extension: None,
			log_id: None,
			inner: None,
		}
	}

	/// An generic unauthorized response.
	///
	/// This is a generic response for someone that tries to access an
	/// authorized resource without proper authorization.
	pub fn unauthorized() -> Self {
		Self::new(
			StatusCode::UNAUTHORIZED,
			String::from("Unauthorized for resource"),
			String::from("You do not have permission to access this resource."),
		)
	}

	/// Adds a custom detail to the [`HandlerError`].
	pub fn with_instance(mut self, instance: impl Into<String>) -> Self
	{
		self.instance = Some(instance.into());
		self
	}

	/// Adds a custom detail to the [`HandlerError`].
	pub fn with_extension<T>(mut self, detail: T) -> Self
	where
		T: Into<D>,
	{
		self.extension = Some(detail.into());
		self
	}

	/// Adds an error to the [`HandlerError`].
	pub fn with_error<E>(mut self, error: E) -> Self
	where
		E: Into<anyhow::Error>,
	{
		self.inner = Some(error.into());
		self
	}

	/// Sets the `log_id` field for the [`HandlerError`].
	///
	/// The `log_id` field is automatically set when the `inner` field is present and the
	/// `log_id` is [`None`]. Changing this field might make it hard or impossible to
	/// track the error or in other ways, break how the error is logged.
	///
	/// # Safety
	///
	/// Make sure you use a globally unique identifier for the `log_id`.
	pub unsafe fn with_log_id<U>(mut self, log_id: U) -> Self
	where
		U: Into<String>,
	{
		self.log_id = Some(log_id.into());
		self
	}
}

impl<D> IntoResponse for HandlerError<D>
where
	D: Serialize + Send + Sync,
{
	/// Converts a [`HandlerError`] into a [`Response`].
	///
	/// This automatically logs errors using [`tracing`]. This also
	/// sets the log_id field so that the error can be tracked.
	fn into_response(mut self) -> Response {
		if let Some(error) = self.inner.as_ref() {
			if self.log_id.is_none() {
				self.log_id = Some(Uuid::new_v4().into())
			};

			let HandlerError {
				ref title,
				ref detail,
				ref log_id,
				..
			} = self;

			let log_id = log_id.as_ref().unwrap(); // `log_id` is guaranteed to be set (above).

			if self.status.is_server_error() {
				tracing::error!(log_id = %log_id, server_error = %error, "An server error occurred");
			} else {
				tracing::error!(log_id = %log_id, client_error = %title, message = %detail, "An client error occurred");
			}
		}

		(
			self.status,
			[("Content-Type", "application/problem+json")],
			Json(self),
		).into_response()
	}
}

impl<D> OperationOutput for HandlerError<D>
where
	D: Serialize + Send + Sync,
{
	type Inner = D;
}

impl<E, D> From<E> for HandlerError<D>
where
	E: Into<anyhow::Error>,
	D: Serialize + Send + Sync,
{
	/// Turns any error into a [`HandlerError`].
	///
	/// This assumes that the error is an internal server error.
	/// This will also set the error in the `inner` field.
	fn from(value: E) -> Self {
		Self {
			status: StatusCode::INTERNAL_SERVER_ERROR,
			title: String::from("Something went wrong"),
			detail: String::from("If this issue persists, please contact an administrator."),
			instance: None,
			extension: None,
			log_id: None, // This will be set in [`HandlerError::into_response()`] if `inner` is [`Some`].
			inner: Some(value.into()),
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[derive(Serialize, Clone)]
	struct Extension {
		field: String,
	}

	#[derive(thiserror::Error, Debug)]
	enum Error {
		#[error("this is a random error")]
		Random,
	}

	#[test]
	fn test_internal_server_error() {
		let extension = Extension {
			field: String::from("This is a random error."),
		};

		let handler_error: HandlerError<Extension> = HandlerError::new(
			StatusCode::BAD_REQUEST,
			"Bad Request",
			"Something went wrong, please contact an developer",
		)
		.with_error(Error::Random)
		.with_extension(extension.clone());

		assert!(handler_error.inner.is_some());
		assert!(handler_error.extension.is_some());
		assert!(handler_error.log_id.is_none()); // `log_id` is set when turned into a response.

		let error_detail = handler_error.extension.as_ref().unwrap();

		assert_eq!(error_detail.field, extension.field);

		let response = handler_error.into_response();

		assert!(response.status().is_client_error());
	}

	#[test]
	fn test_any_error_to_handler_result() {
		let example_handler = || -> HandlerResult<i32> { Ok("abc".parse::<i32>()?) };

		let handler_error = example_handler().unwrap_err();

		assert!(handler_error.status.is_server_error());
		assert!(handler_error.inner.is_some());
		assert!(handler_error.log_id.is_none()); // `log_id` is set when turned into a response.
	}

	#[test]
	fn test_unsafe_set_log_id() {
		let example_handler_one = || -> HandlerResult<i32> { Ok("abc".parse::<i32>()?) };
		let example_handler_two = || -> HandlerResult<f64> { Ok("xyz".parse::<f64>()?) };
		let example_handler_three = || -> HandlerResult<i16> { Ok("qwe".parse::<i16>()?) };

		let handler_error_one = unsafe {
			example_handler_one()
				.unwrap_err()
				.with_log_id("example_log_id")
		};
		let handler_error_two = unsafe {
			example_handler_two()
				.unwrap_err()
				.with_log_id("example_log_id")
		};
		let handler_error_three = example_handler_three().unwrap_err();

		assert!(handler_error_one.log_id.is_some());
		assert!(handler_error_two.log_id.is_some());
		assert!(handler_error_three.log_id.is_none());
		assert_eq!(handler_error_one.log_id, handler_error_two.log_id)
	}
}
