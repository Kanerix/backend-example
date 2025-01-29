use aide::{axum::IntoApiResponse, transform::TransformOperation};
use axum::{extract::{Path, State}, Json};
use sqlx::PgPool;

use crate::{error::HandlerResult, middleware::AuthUser};

use super::{CommentParams, CommentRequest};

#[axum::debug_handler]
pub async fn handler(
    Path(params): Path<CommentParams>,
	AuthUser(user): AuthUser,
	State(pool): State<PgPool>,
	Json(comment): Json<CommentRequest>,
) -> HandlerResult<impl IntoApiResponse> {
    sqlx::query!(
		"UPDATE comments SET body = $1
		WHERE user_id = $2 AND post_id = $3 AND id = $4",
		&comment.body,
		&user.id,
		&params.post_id,
		&params.comment_id,
	)
	.execute(&pool)
	.await?;

	Ok(())
}

pub fn docs(op: TransformOperation) -> TransformOperation {
	op.description("Edit a comment.").tag("posts")
}
