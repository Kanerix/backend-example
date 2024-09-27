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
        Auth::login,
        Auth::register,
        Health::health,
    ),
    components(schemas(
        Auth::LoginRequest,
        Auth::LoginResponse,
        Auth::RegisterRequest,
    )),
    tags(
        (name = "Auth", description = "Endpoints for handling user authentication."),
        (name = "Health", description = "Endpoints checking application health."),
    )
)]
pub struct ApiDoc;
