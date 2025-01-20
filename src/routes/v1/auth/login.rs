use aide::axum::IntoApiResponse;
use axum::{extract::State, Json};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
	error::{HandlerError, HandlerResult},
	models,
	utils::{
		pwd::validate_pwd,
		token::{generate_access_token, generate_refresh_token, TokenUser},
	},
};

use super::TokenResponse;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct LoginRequest {
	username: String,
	password: String,
}

pub struct UserWithPassword {
	pub id: Uuid,
	pub username: String,
	pub email: String,
	pub role: models::UserRole,
	pub hash: String,
	pub salt: Option<String>,
	pub created_at: DateTime<Utc>,
}

impl From<&UserWithPassword> for TokenUser {
	fn from(user: &UserWithPassword) -> TokenUser {
		TokenUser {
			id: user.id,
			username: user.username.to_owned(),
			email: user.email.to_owned(),
			role: user.role.to_owned(),
		}
	}
}

#[axum::debug_handler]
pub async fn login(
	State(pool): State<PgPool>,
	Json(payload): Json<LoginRequest>,
) -> HandlerResult<impl IntoApiResponse> {
	let user = sqlx::query_as!(
		UserWithPassword,
		"SELECT
        u.id,
        u.email,
        u.username,
        u.role AS \"role: models::UserRole\",
        u.created_at,
        p.hash,
        p.salt
        FROM users u
        INNER JOIN passwords p ON u.id = p.user_id
        WHERE email = $1",
		&payload.username,
	)
	.fetch_one(&pool)
	.await
	.map_err(|err| match err {
		sqlx::Error::RowNotFound => HandlerError::unauthorized(),
		_ => HandlerError::from(err),
	})?;

	if !validate_pwd(&user.hash, &payload.password, user.salt.as_deref()).await? {
		return Err(HandlerError::unauthorized());
	}

	let refresh_token = generate_refresh_token();
	let access_token = generate_access_token(&user)?;

	sqlx::query!(
		"INSERT INTO refresh_tokens ( user_id, token ) VALUES ($1, $2)",
		&user.id,
		refresh_token
	)
	.execute(&pool)
	.await?;

	let refresh_cookie = Cookie::build(("refresh_token", refresh_token))
		.path("/")
		.secure(true)
		.http_only(true);

	Ok((
		CookieJar::new().add(refresh_cookie),
		Json(TokenResponse {
			kind: "Bearer".into(),
			token: access_token,
		}),
	))
}
