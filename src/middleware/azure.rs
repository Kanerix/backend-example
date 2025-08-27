use axum::{
	extract::{FromRef, FromRequestParts},
	http::request::Parts,
};
use jsonwebtoken::{jwk::JwkSet, DecodingKey};

use crate::error::HandlerError;

pub struct AzureUser;

#[derive(Clone, FromRef)]
pub struct AzureConfig {
	tenant_id: &'static str,
}

impl<S> FromRequestParts<S> for AzureUser
where
	AzureConfig: FromRef<S>,
	S: Send + Sync,
{
	type Rejection = HandlerError;

	async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
		let config = AzureConfig::from_ref(state);
		let jwk_set = JwkSet::

		Ok(AzureUser {})
	}
}
