use aide::axum::IntoApiResponse;
use axum::{
	extract::{Path, State},
	Json,
};
use sqlx::PgPool;

use crate::{error::HandlerResult, middleware::AuthUser};

use super::{PostParams, PostRequest};

#[axum::debug_handler]
pub async fn edit(
	Path(params): Path<PostParams>,
	AuthUser(user): AuthUser,
	State(pool): State<PgPool>,
	Json(post): Json<PostRequest>,
) -> HandlerResult<impl IntoApiResponse> {
	sqlx::query_as!(
		models::Post,
		"UPDATE posts
		SET title = $1, body = $2
		WHERE user_id = $3 AND id = $4",
		&post.title,
		&post.body,
		&user.id,
		&params.post_id
	)
	.execute(&pool)
	.await?;

	Ok(())
}
