mod auth;
mod health;
mod post;

use auth as Auth;
use axum::{routing::get, Router};
use health as Health;

use sqlx::PgPool;
use utoipa::OpenApi;

const AUTH_TAG: &str = "Authentication API endpoints";
const POSTS_TAG: &str = "Post API endpoints";
const HEALTH_TAG: &str = "Health API endpoints";

pub fn routes() -> Router<PgPool> {
	Router::new()
		.route("/health", get(health::health))
		.nest("/auth", Auth::routes())
		.nest("/post", post::routes())
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
        Auth::RegisterRequest,
        Auth::TokenResponse,
    )),
    tags(
        (name = AUTH_TAG, description = "Endpoints for user authentication."),
        (name = POSTS_TAG, description = "Endpoints for handling posts."),
        (name = HEALTH_TAG, description = "Endpoints for checking application health."),
    )
)]
pub struct ApiDoc;
