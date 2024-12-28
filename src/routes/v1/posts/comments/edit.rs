use axum::response::IntoResponse;

use crate::{error::HandlerResult, routes::v1::POSTS_TAG};

#[utoipa::path(
	patch,
	path = "/api/v1/posts/{post_id}/comments/{comment_id}/edit",
	responses(
        (status = 200, description = "List of comments on the post"),
    ),
    params(
        ("post_id" = Uuid, Path, description = "The UUID of the post")
    ),
    tag = POSTS_TAG
)]
pub async fn edit() -> HandlerResult<impl IntoResponse> {
	Ok(())
}
