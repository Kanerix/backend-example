use aide::axum::IntoApiResponse;
use axum::{extract::State, Json};
use axum_extra::extract::CookieJar;
use sqlx::PgPool;

use crate::{
	error::{HandlerError, HandlerResult},
	models,
	utils::token::generate_access_token,
};

use super::TokenResponse;

#[axum::debug_handler]
pub async fn refresh(
	jar: CookieJar,
	State(pool): State<PgPool>,
) -> HandlerResult<impl IntoApiResponse> {
	let refresh_cookie = jar
		.get("refresh_token")
		.ok_or(HandlerError::unauthorized())?;
	let refresh_token = refresh_cookie.value();

	let user = sqlx::query_as!(
		models::User,
		"SELECT
        u.id,
        u.email,
        u.username,
        u.role AS \"role: models::UserRole\",
        u.created_at
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

	Ok(Json(TokenResponse {
		kind: "Bearer".into(),
		token: access_token,
	}))
}
