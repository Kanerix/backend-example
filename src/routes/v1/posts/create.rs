use aide::{axum::IntoApiResponse, transform::TransformOperation};
use axum::{extract::State, Json};

use crate::{error::HandlerResult, middleware::AuthUser, AppState};

use super::PostRequest;

#[axum::debug_handler]
pub async fn handler(
	AuthUser(user): AuthUser,
	State(state): State<AppState>,
	Json(post): Json<PostRequest>,
) -> HandlerResult<impl IntoApiResponse> {
	sqlx::query!(
		"INSERT INTO posts (user_id, title, body) VALUES ($1, $2, $3)",
		&user.id,
		&post.title,
		&post.body,
	)
	.execute(&state.pg)
	.await?;

	Ok(())
}

pub fn docs(op: TransformOperation) -> TransformOperation {
	op.description("Create a post.").tag("posts")
}
