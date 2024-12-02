use axum::response::IntoResponse;

use crate::{error::HandlerResult, routes::v1::POSTS_TAG};

#[utoipa::path(
	post,
	path = "/api/v1/posts/{post_id}/comment",
	responses(
        (status = 200, description = "Succefully added comment to post"),
    ),
    params(
        ("post_id" = Uuid, Path, description = "The UUID of the post")
    ),
    tag = POSTS_TAG
)]
pub async fn comment() -> HandlerResult<impl IntoResponse> {
	Ok(())
}
