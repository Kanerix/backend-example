use aide::{axum::IntoApiResponse, transform::TransformOperation};
use axum::{extract::{Path, State}, Json};

use crate::{error::HandlerResult, middleware::AuthUser, AppState};

use super::{CommentParams, CommentRequest};

#[axum::debug_handler]
pub async fn handler(
    Path(params): Path<CommentParams>,
	AuthUser(user): AuthUser,
	State(state): State<AppState>,
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
	.execute(&state.pg)
	.await?;

	Ok(())
}

pub fn docs(op: TransformOperation) -> TransformOperation {
	op.description("Edit a comment.").tag("posts")
}
