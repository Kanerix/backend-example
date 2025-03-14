use aide::{axum::IntoApiResponse, transform::TransformOperation};
use axum::{extract::State, http::StatusCode, Json};
use schemars::JsonSchema;
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use crate::{
	error::{HandlerError, HandlerResult},
	middleware::Validated,
	models,
	utils::pwd::{self, hash_pwd},
	AppState,
};

#[derive(Debug, Deserialize, JsonSchema, Validate)]
pub struct RegisterRequest {
	#[validate(email)]
	email: String,
	#[validate(length(min = 3, max = 32))]
	username: String,
	#[validate(
		length(min = 8, max = 64),
		regex(path = "pwd::PASSWORD_VALIDATION_REGEX")
	)]
	password: String,
}

#[axum::debug_handler]
pub async fn handler(
	State(state): State<AppState>,
	Validated(Json(payload)): Validated<Json<RegisterRequest>>,
) -> HandlerResult<impl IntoApiResponse> {
	let salt = Uuid::new_v4().to_string();
	let password = payload.password;
	let hash = hash_pwd(&password, &salt).await?;

	let mut tx = state.pg.begin().await?;

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
