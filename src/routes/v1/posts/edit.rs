use axum::{extract::{Path, State}, response::IntoResponse, Json};
use sqlx::PgPool;

use crate::{error::HandlerResult, middleware::AuthUser, routes::v1::POSTS_TAG};

use super::{PostParams, PostRequest};

#[utoipa::path(
	patch,
	path = "/api/v1/posts/{post_id}/edit",
	responses(
        (status = 200, description = "Successfully edited post"),
    ),
    params(
        ("post_id" = Uuid, Path, description = "The UUID of the post")
    ),
    tag = POSTS_TAG
)]
#[axum::debug_handler]
pub async fn edit(
    Path(params): Path<PostParams>,
    AuthUser(user): AuthUser,
    State(pool): State<PgPool>,
    Json(post): Json<PostRequest>,
) -> HandlerResult<impl IntoResponse> {
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
