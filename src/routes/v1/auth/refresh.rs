use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::CookieJar;
use sqlx::PgPool;

use crate::{
	error::{HandlerError, HandlerResult},
	models,
	routes::v1::AUTH_TAG,
	utils::token::generate_access_token,
};

use super::TokenResponse;

#[utoipa::path(
	get,
	path = "/api/v1/auth/refresh",
	responses(
        (status = 200, description = "Successful refresh", body = TokenResponse),
    ),
    tag = AUTH_TAG
)]
#[axum::debug_handler]
pub async fn refresh(
    jar: CookieJar,
	State(pool): State<PgPool>,
) -> HandlerResult<impl IntoResponse> {
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
