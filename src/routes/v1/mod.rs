pub mod auth;
pub mod health;

use auth as Auth;
use axum::{routing::get, Router};
use health as Health;

use sqlx::PgPool;
use utoipa::OpenApi;

const AUTH_TAG: &str = "Authentication API endpoints";
const HEALTH_TAG: &str = "Health API endpoint";

pub fn routes() -> Router<PgPool> {
	Router::new()
		.route("/health", get(health::health))
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
        (name = AUTH_TAG, description = "Endpoints for handling user authentication."),
        (name = HEALTH_TAG, description = "Endpoints for checking application health."),
    )
)]
pub struct ApiDoc;
