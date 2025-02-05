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

use crate::AppState;

pub fn routes(state: AppState) -> ApiRouter {
	let router = ApiRouter::new()
		.route("/swagger", Swagger::new("/api/v1/api.json").axum_route())
		.route("/api.json", get(serve_api))
		.api_route("/health", get_with(health::handler, health::docs))
		.nest_api_service("/auth", auth::routes(state.clone()))
		.nest_api_service("/posts", posts::routes(state.clone()));

	#[cfg(debug_assertions)]
	let router = router.api_route("/failure", get_with(failure::handler, failure::docs));

	router
}

async fn serve_api(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
	Json(api)
}
