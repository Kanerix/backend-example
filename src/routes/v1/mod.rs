pub mod auth;
pub mod health;

use auth as Auth;
use health as Health;

use axum::{routing::get, Router};
use sqlx::PgPool;
use utoipa::OpenApi;

pub fn routes() -> Router<PgPool> {
	Router::new()
		.route("/health", get(Health::health))
		.nest("/auth", Auth::routes())
}

#[derive(OpenApi)]
#[openapi(
    paths(
        Auth::login::login,
        Auth::register::register,
        Auth::refresh::refresh,
        Health::health,
    ),
    components(schemas(
        Auth::LoginRequest,
        Auth::LoginResponse,
        Auth::RegisterRequest,
        Auth::RefreshResponse,
    )),
    tags(
        (name = "Auth", description = "Endpoints for handling user authentication."),
        (name = "Health", description = "Endpoints for checking application health."),
    )
)]
pub struct ApiDoc;
