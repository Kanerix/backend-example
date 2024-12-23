use axum::{
	extract::{Path, State},
	response::IntoResponse,
};
use sqlx::PgPool;

use crate::{error::HandlerResult, middleware::AuthUser, routes::v1::POSTS_TAG};

use super::PostParams;

#[utoipa::path(
	delete,
	path = "/api/v1/posts/{post_id}/delete",
	responses(
        (status = 200, description = "Successfully deleted post"),
    ),
    params(
        ("post_id" = Uuid, Path, description = "The UUID of the post")
    ),
    tag = POSTS_TAG
)]
pub async fn destroy(
	Path(params): Path<PostParams>,
	AuthUser(user): AuthUser,
	State(pool): State<PgPool>,
) -> HandlerResult<impl IntoResponse> {
	sqlx::query_as!(
		models::Post,
		"DELETE FROM posts WHERE user_id = $1 AND id = $2",
		&user.id,
		&params.post_id
	)
	.execute(&pool)
	.await?;

	Ok(())
}
