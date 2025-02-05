use aide::{axum::IntoApiResponse, transform::TransformOperation};
use axum::{extract::{Path, State}, Json};

use crate::{error::HandlerResult, middleware::AuthUser, routes::v1::posts::PostParams, AppState};

use super::CommentRequest;

#[axum::debug_handler]
pub async fn handler(
    Path(params): Path<PostParams>,
    AuthUser(user): AuthUser,
	State(state): State<AppState>,
	Json(comment): Json<CommentRequest>,
) -> HandlerResult<impl IntoApiResponse> {
    sqlx::query!(
		"INSERT INTO comments (user_id, post_id, body) VALUES ($1, $2, $3)",
		&user.id,
		&params.post_id,
		&comment.body,
	)
	.execute(&state.pg)
	.await?;

	Ok(())
}

pub fn docs(op: TransformOperation) -> TransformOperation {
	op.description("Create a comment.").tag("posts")
}
