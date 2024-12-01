/// All routes used for authentication and authorization.
pub mod login;
pub mod refresh;
pub mod register;

use axum::{routing::post, Router};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub use login::{login, LoginRequest};
pub use refresh::refresh;
pub use register::{register, RegisterRequest};
use utoipa::ToSchema;

pub fn routes() -> Router<PgPool> {
	Router::new()
		.route("/login", post(login))
		.route("/refresh", post(refresh))
		.route("/register", post(register))
}

/// Response object for endpoints that return a token.
#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenResponse {
	pub kind: String,
	pub token: String,
}
