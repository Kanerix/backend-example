use aide::OperationInput;
use axum::{
	extract::FromRequestParts,
	http::{header, request::Parts},
};

use crate::{
	error::HandlerError,
	utils::token::{decode_access_token, Error::TokenError, TokenUser},
};

/// An authenticated user.
///
/// This will authorize the user based on the provided
/// token given in the `Authorization` header.
pub struct AuthUser(pub TokenUser);

impl<S> FromRequestParts<S> for AuthUser
where
	S: Send + Sync,
{
	type Rejection = HandlerError;

	async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
		let token = parts
			.headers
			.get(header::AUTHORIZATION)
			.and_then(|header| header.to_str().ok())
			.ok_or(HandlerError::unauthorized())?
			.split_whitespace()
			.last()
			.ok_or(HandlerError::unauthorized())?;

		let token_data = decode_access_token(token).map_err(|err| match err {
		    TokenError(_) => HandlerError::unauthorized(),
		})?;

		Ok(AuthUser(token_data.claims.user))
	}
}

impl OperationInput for AuthUser {}
