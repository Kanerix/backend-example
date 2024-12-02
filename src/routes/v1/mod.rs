mod auth;
mod health;
mod posts;

use auth as Auth;
use axum::{routing::get, Router};
use health as Health;
use posts as Posts;

use sqlx::PgPool;
use utoipa::OpenApi;

const AUTH_TAG: &str = "Authentication API";
const POSTS_TAG: &str = "Post API";
const HEALTH_TAG: &str = "Health API";

pub fn routes() -> Router<PgPool> {
	Router::new()
		.route("/health", get(health::health))
		.nest("/auth", Auth::routes())
		.nest("/post", Posts::routes())
}

#[derive(OpenApi)]
#[openapi(
    paths(
        Auth::login::login,
        Auth::register::register,
        Auth::refresh::refresh,
        Posts::comment::comment,
        Posts::create::create,
        Posts::delete::destroy,
        Posts::edit::edit,
        Posts::list::list,
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
