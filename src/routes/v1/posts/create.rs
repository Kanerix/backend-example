use axum::{extract::State, response::IntoResponse, Json};
use sqlx::PgPool;

use crate::{error::HandlerResult, middleware::AuthUser, routes::v1::POSTS_TAG};

use super::PostRequest;

#[utoipa::path(
	post,
	path = "/api/v1/posts/create",
	request_body(
        content = PostRequest,
        description = "An object containing the title and body of the post.",
        content_type = "application/json"
    ),
	responses(
        (status = 200, description = "Successfully created post"),
    ),
    tag = POSTS_TAG
)]
#[axum::debug_handler]
pub async fn create(
    AuthUser(user): AuthUser,
    State(pool): State<PgPool>,
    Json(post): Json<PostRequest>,
) -> HandlerResult<impl IntoResponse> {
	sqlx::query_as!(
		models::Post,
		"INSERT INTO posts (user_id, title, body)
		VALUES ($1, $2, $3)",
		&user.id,
		&post.title,
		&post.body,
	)
	.execute(&pool)
	.await?;

	Ok(())
}
