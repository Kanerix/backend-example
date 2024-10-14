pub mod login;
pub mod register;

use axum::{routing::post, Router};
use sqlx::PgPool;

pub use login::{login, LoginRequest, LoginResponse};
pub use register::{register, RegisterRequest};

pub fn routes() -> Router<PgPool> {
	Router::new()
		.route("/login", post(login))
		.route("/register", post(register))
}
