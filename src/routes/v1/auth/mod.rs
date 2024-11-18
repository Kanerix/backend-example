/// All routes used for authentication and authorization.
pub mod login;
pub mod refresh;
pub mod register;

use axum::{routing::post, Router};
use sqlx::PgPool;

pub use login::{LoginRequest, LoginResponse};
pub use refresh::RefreshResponse;
pub use register::RegisterRequest;

pub fn routes() -> Router<PgPool> {
	Router::new()
		.route("/login", post(login::login))
		.route("/register", post(register::register))
		.route("/refresh", post(refresh::refresh))
}
