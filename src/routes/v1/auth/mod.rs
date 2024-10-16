/// All routes used for authentication and authorization.
pub mod login;
pub mod refresh;
pub mod register;

use axum::{
	routing::{get, post},
	Router,
};
use sqlx::PgPool;

pub use login::{login, LoginRequest, LoginResponse};
pub use refresh::refresh;
pub use register::{register, RegisterRequest};

pub fn routes() -> Router<PgPool> {
	Router::new()
		.route("/login", post(login))
		.route("/register", post(register))
		.route("/refresh", get(refresh))
}
