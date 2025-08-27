use std::borrow::Cow;

use axum::{
	extract::{FromRef, FromRequestParts},
	http::request::Parts,
};

use crate::error::HandlerError;

pub struct AzureUser;

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
		let config = AzureConfig::from_ref(state);
		let req = reqwest::Client::new()
			.get(format!(
				"https://login.microsoftonline.com/{}/discovery/v2.0/keys",
				config.tenant_id
			))
			.send()
			.await
			.map_err(|e| HandlerError::new(500, format!("Failed to fetch JWKs: {}", e)))?;

		Ok(AzureUser {})
	}
}
