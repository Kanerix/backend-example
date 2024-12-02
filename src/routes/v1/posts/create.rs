use axum::response::IntoResponse;

use crate::{error::HandlerResult, routes::v1::POSTS_TAG};

#[utoipa::path(
	post,
	path = "/api/v1/posts/create",
	responses(
        (status = 200, description = "Successfully created post"),
    ),
    tag = POSTS_TAG
)]
pub async fn create() -> HandlerResult<impl IntoResponse> {
	Ok(())
}
