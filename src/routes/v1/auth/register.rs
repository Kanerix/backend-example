use aide::{axum::IntoApiResponse, transform::TransformOperation};
use axum::{extract::State, http::StatusCode, Json};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
	error::{HandlerError, HandlerResult},
	models,
	utils::pwd::hash_pwd,
};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct RegisterRequest {
	email: String,
	username: String,
	password: String,
}

#[axum::debug_handler]
pub async fn handler(
	State(pool): State<PgPool>,
	Json(payload): Json<RegisterRequest>,
) -> HandlerResult<impl IntoApiResponse> {
	let salt = Uuid::new_v4().to_string();
	let password = payload.password;
	let hash = hash_pwd(&password, &salt).await?;

	let mut tx = pool.begin().await?;

	let user = sqlx::query_as!(
		models::User,
		"INSERT INTO users ( email, username )
        VALUES ($1, $2)
        RETURNING
        users.id,
        users.email,
        users.username,
        users.role AS \"role: models::UserRole\",
        users.created_at",
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
		"INSERT INTO passwords ( hash, salt, user_id )
		VALUES ($1, $2, $3)",
		&hash,
		&salt,
		&user.id,
	)
	.execute(&mut *tx)
	.await?;

	tx.commit().await?;

	Ok(())
}

pub fn docs(op: TransformOperation) -> TransformOperation {
	op.description("Register an account.").tag("auth")
}
