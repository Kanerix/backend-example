use std::borrow::Cow;

use axum::{
	extract::{FromRef, FromRequestParts},
	http::request::Parts,
};
use jsonwebtoken::{decode, decode_header, jwk::JwkSet, DecodingKey};
use serde::Deserialize;

use crate::error::HandlerError;

#[derive(Deserialize)]
pub struct AzureUser {
	pub upn: String,
}

#[derive(Clone, FromRef)]
pub struct AzureConfig {
	tenant_id: Option<Cow<'static, str>>,
}

impl AzureConfig {
	pub fn get_key_discovery_url(&self) -> String {
		format!(
			"https://login.microsoftonline.com/{}/discovery/v2.0/keys",
			self.tenant_id.as_deref().unwrap_or("common")
		)
	}
}

impl<S> FromRequestParts<S> for AzureUser
where
	AzureConfig: FromRef<S>,
	S: Send + Sync,
{
	type Rejection = HandlerError;

	async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
		let token = parts
			.headers
			.get("Authorization")
			.and_then(|h| h.to_str().ok())
			.and_then(|h| h.strip_prefix("Bearer "))
			.ok_or_else(|| HandlerError::unauthorized())?;

		let decoded_token = decode_header(token).map_err(|_| HandlerError::unauthorized())?;
		let kid = decoded_token
			.kid
			.ok_or_else(|| HandlerError::unauthorized())?;

		let config = AzureConfig::from_ref(state);
		let req = reqwest::Client::new()
			.get(config.get_key_discovery_url())
			.send()
			.await
			.map_err(|_| HandlerError::unauthorized())?;

		let jwk_set = req
			.json::<JwkSet>()
			.await
			.map_err(|_| HandlerError::unauthorized())?;
		let jwk = jwk_set
			.find(&kid)
			.ok_or_else(|| HandlerError::unauthorized())?;

		let decoding_key = DecodingKey::from_jwk(jwk).map_err(|_| HandlerError::unauthorized())?;

		let token_data = decode::<AzureUser>(token, &decoding_key, &jsonwebtoken::Validation::default())
			.map_err(|_| HandlerError::unauthorized())?;

		Ok(token_data.claims)
	}
}
