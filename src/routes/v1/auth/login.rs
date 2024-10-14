use axum::{extract::State, http::StatusCode, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
	error::{HandlerError, HandlerResult},
	models,
	utils::{pwd::validate_pwd, token::generate_access_token},
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
) -> HandlerResult<Json<LoginResponse>> {
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

	let user = sqlx::query_as!(
		UserWithPassword,
		"SELECT
        users.id,
        users.email,
        users.username,
        users.role AS \"role: models::user::UserRole\",
        users.created_at,
        users.updated_at,
        passwords.hash,
        passwords.salt
        FROM users
        INNER JOIN passwords ON users.id = passwords.user_id
        WHERE email = $1",
		&payload.username,
	)
	.fetch_one(&pool)
	.await
	.map_err(|err| match err {
		sqlx::Error::RowNotFound => HandlerError::new(
			StatusCode::NOT_FOUND,
			"User not found",
			format!(
				"No user with the username \"{}\" was found",
				payload.username
			),
		),
		sqlx::Error::Database(db_err) => match db_err.kind() {
			sqlx::error::ErrorKind::UniqueViolation => HandlerError::new(
				StatusCode::CONFLICT,
				"Unique violation",
				"Email or username already exsits",
			),
			_ => HandlerError::from(db_err),
		},
		_ => HandlerError::from(err),
	})?;

	if !validate_pwd(user.hash, payload.password, user.salt).await? {
		return Err(HandlerError::unauthorized());
	}

	let user = models::user::User {
		id: user.id,
		username: user.username,
		email: user.email,
		role: user.role,
		created_at: user.created_at,
		updated_at: user.updated_at,
	};

	let token = generate_access_token(user)?;

	Ok(Json(LoginResponse {
		kind: "Bearer".into(),
		token,
	}))
}
