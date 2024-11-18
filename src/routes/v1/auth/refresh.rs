use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::ToSchema;

use crate::{
	error::{HandlerError, HandlerResult},
	models,
	routes::v1::AUTH_TAG,
	utils::token::generate_access_token,
};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct RefreshResponse {
	kind: String,
	token: String,
}

#[utoipa::path(
	post,
	path = "/api/v1/auth/refresh",
	request_body(
        content = RefreshResponse,
		description = "Refresh your access token using a refresh token",
		content_type = "application/json"
	),
	responses(
        (status = 200, description = "Successful refresh", body = RefreshResponse),
    ),
    tag = AUTH_TAG
)]
pub async fn refresh(
	State(pool): State<PgPool>,
	jar: CookieJar,
) -> HandlerResult<impl IntoResponse> {
	let refresh_cookie = jar
		.get("refresh_token")
		.ok_or(HandlerError::unauthorized())?;
	let refresh_token = refresh_cookie.value();

	let user = sqlx::query_as!(
		models::user::User,
		"SELECT
        u.id,
        u.email,
        u.username,
        u.role AS \"role: models::user::UserRole\",
        u.created_at,
        u.updated_at
        FROM users u
        JOIN refresh_tokens t ON u.id = t.user_id
        WHERE expires_at > NOW() AND token = $1",
		refresh_token,
	)
	.fetch_one(&pool)
	.await
	.map_err(|err| match err {
		sqlx::Error::RowNotFound => HandlerError::unauthorized(),
		_ => HandlerError::from(err),
	})?;

	let access_token = generate_access_token(user)?;

	Ok(Json(RefreshResponse {
		kind: "Bearer".into(),
		token: access_token,
	}))
}
