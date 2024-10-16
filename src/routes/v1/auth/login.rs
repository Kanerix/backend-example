use axum::{extract::State, response::IntoResponse, Json};
use axum_extra::extract::{cookie::Cookie, CookieJar};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
	error::{HandlerError, HandlerResult},
	models,
	utils::{
		pwd::validate_pwd,
		token::{claims::TokenUser, generate_access_token, generate_refresh_token},
	},
};

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct LoginRequest {
	username: String,
	password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
	kind: String,
	token: String,
}

pub struct UserWithPassword {
	pub id: Uuid,
	pub username: String,
	pub email: String,
	pub role: models::user::UserRole,
	pub hash: String,
	pub salt: Option<String>,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

impl From<&UserWithPassword> for TokenUser {
	fn from(user: &UserWithPassword) -> TokenUser {
		TokenUser {
			id: user.id,
			username: user.username.clone(),
			email: user.email.clone(),
			role: user.role.clone(),
		}
	}
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body(
        content = LoginRequest,
        description = "An object containing the username and password of the user.",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Successful login", body = LoginResponse),
    ),
)]
pub async fn login(
	State(pool): State<PgPool>,
	Json(payload): Json<LoginRequest>,
) -> HandlerResult<impl IntoResponse> {
	let user = sqlx::query_as!(
		UserWithPassword,
		"SELECT
        u.id,
        u.email,
        u.username,
        u.role AS \"role: models::user::UserRole\",
        u.created_at,
        u.updated_at,
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
		Json(LoginResponse {
			kind: "Bearer".into(),
			token: access_token,
		}),
	))
}
