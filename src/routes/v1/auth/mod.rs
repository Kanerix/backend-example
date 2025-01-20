//! All routes for authentication and authorization.

pub mod login;
pub mod refresh;
pub mod register;

use aide::axum::{routing::post_with, ApiRouter};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub fn routes(state: PgPool) -> ApiRouter {
	ApiRouter::new()
		.api_route("/login", post_with(login::handler, login::docs))
		.api_route("/refresh", post_with(refresh::handler, refresh::docs))
		.api_route("/register", post_with(register::handler, register::docs))
		.with_state(state)
}

/// Response object for endpoints that return a token.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TokenResponse {
	pub kind: String,
	pub token: String,
}
