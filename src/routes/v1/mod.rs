mod auth;
mod health;
mod posts;

#[cfg(debug_assertions)]
mod failure;

use aide::axum::routing::get;
use aide::axum::ApiRouter;
use aide::axum::IntoApiResponse;
use aide::openapi::OpenApi;
use aide::swagger::Swagger;
use axum::Extension;
use axum::Json;

use auth as Auth;
use posts as Posts;

use sqlx::PgPool;

pub fn routes() -> ApiRouter<PgPool> {
	let router = ApiRouter::new()
		.route("/swagger", Swagger::new("/api/v1/api.json").axum_route())
		.route("/api.json", get(serve_api))
		.api_route("/health", get(health::health))
		.nest("/auth", Auth::routes())
		.nest("/post", Posts::routes());

	#[cfg(debug_assertions)]
	let router = router.route("/failure", get(failure::failure));

	router
}

pub struct ApiDoc;

async fn serve_api(Extension(api): Extension<OpenApi>) -> impl IntoApiResponse {
	Json(api)
}
