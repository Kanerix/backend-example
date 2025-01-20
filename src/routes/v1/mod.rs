mod auth;
mod health;
mod posts;

#[cfg(debug_assertions)]
mod failure;

use std::sync::Arc;

use aide::axum::routing::get;
use aide::axum::routing::get_with;
use aide::axum::ApiRouter;
use aide::axum::IntoApiResponse;
use aide::openapi::OpenApi;
use aide::swagger::Swagger;
use axum::Extension;
use axum::Json;

use auth as auth_service;
use posts as posts_service;

use sqlx::PgPool;

pub fn routes(state: PgPool) -> ApiRouter {
	let router = ApiRouter::new()
		.route("/swagger", Swagger::new("/api/v1/api.json").axum_route())
		.route("/api.json", get(serve_api))
		.api_route("/health", get_with(health::handler, health::docs))
		.nest_api_service("/auth", auth_service::routes(state.clone()))
		.nest_api_service("/posts", posts_service::routes(state.clone()));

	#[cfg(debug_assertions)]
	let router = router.api_route("/failure", get_with(failure::handler, failure::docs));

	router
}

pub struct ApiDoc;

async fn serve_api(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
	Json(api)
}
