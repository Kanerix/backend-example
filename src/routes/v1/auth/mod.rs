//! All routes for authentication and authorization.

pub mod login;
pub mod refresh;
pub mod register;

use aide::axum::{routing::post, ApiRouter};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub use login::login;
pub use refresh::refresh;
pub use register::register;

pub fn routes() -> ApiRouter<PgPool> {
	ApiRouter::new()
		.api_route("/login", post(login))
		.api_route("/refresh", post(refresh))
		.api_route("/register", post(register))
}

/// Response object for endpoints that return a token.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct TokenResponse {
	pub kind: String,
	pub token: String,
}
