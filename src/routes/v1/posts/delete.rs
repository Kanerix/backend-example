use aide::axum::IntoApiResponse;
use axum::extract::{Path, State};
use sqlx::PgPool;

use crate::{error::HandlerResult, middleware::AuthUser};

use super::PostParams;

#[axum::debug_handler]
pub async fn destroy(
	Path(params): Path<PostParams>,
	AuthUser(user): AuthUser,
	State(pool): State<PgPool>,
) -> HandlerResult<impl IntoApiResponse> {
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
