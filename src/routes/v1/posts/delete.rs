use aide::{axum::IntoApiResponse, transform::TransformOperation};
use axum::extract::{Path, State};
use sqlx::PgPool;

use crate::{error::HandlerResult, middleware::AuthUser};

use super::PostParams;

#[axum::debug_handler]
pub async fn handler(
	Path(params): Path<PostParams>,
	AuthUser(user): AuthUser,
	State(pool): State<PgPool>,
) -> HandlerResult<impl IntoApiResponse> {
	sqlx::query!(
		"DELETE FROM posts WHERE user_id = $1 AND id = $2",
		&user.id,
		&params.post_id
	)
	.execute(&pool)
	.await?;

	Ok(())
}

pub fn docs(op: TransformOperation) -> TransformOperation {
	op.description("Delete a post.").tag("posts")
}
