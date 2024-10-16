use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

use crate::{
	error::{HandlerError, HandlerResult},
	models,
	utils::pwd::hash_pwd,
};

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct RegisterRequest {
	email: String,
	username: String,
	password: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    request_body(
        content = RegisterRequest,
        description = "An object containing the register payload.",
        content_type = "application/json"
    ),
    responses(
        (status = 200, description = "Account created", body = String),
    ),
)]
pub async fn register(
	State(pool): State<PgPool>,
	Json(payload): Json<RegisterRequest>,
) -> HandlerResult<impl IntoResponse> {
	let salt = Uuid::new_v4().to_string();
	let password = payload.password;
	let hash = hash_pwd(&password, &salt).await?;

	let mut tx = pool.begin().await?;

	let user = sqlx::query_as!(
		models::user::User,
		"INSERT INTO users ( email, username )
        VALUES ($1, $2)
        RETURNING
        users.id,
        users.email,
        users.username,
        users.role AS \"role: models::user::UserRole\",
        users.created_at,
        users.updated_at",
		&payload.email,
		&payload.username,
	)
	.fetch_one(&mut *tx)
	.await
	.map_err(|err| match err {
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

	sqlx::query!(
		"INSERT INTO passwords ( hash, salt, user_id ) VALUES ($1, $2, $3)",
		&hash,
		&salt,
		&user.id,
	)
	.execute(&mut *tx)
	.await?;

	tx.commit().await?;

	Ok(())
}
