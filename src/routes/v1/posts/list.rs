use axum::response::IntoResponse;

use crate::{error::HandlerResult, routes::v1::POSTS_TAG};

#[utoipa::path(
	get,
	path = "/api/v1/posts",
	responses(
        (status = 200, description = "List of posts"),
    ),
    tag = POSTS_TAG
)]
pub async fn list() -> HandlerResult<impl IntoResponse> {
	Ok(())
}
