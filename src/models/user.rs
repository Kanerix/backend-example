//! Users database models

use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
	pub id: Uuid,
	pub username: String,
	pub email: String,
	pub role: UserRole,
	pub created_at: DateTime<Utc>,
}

/// User role model
#[derive(sqlx::Type, Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[sqlx(type_name = "role", rename_all = "lowercase")]
pub enum UserRole {
	ADMIN,
	MODERATOR,
	USER,
}

/// User password model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Password {
	pub id: Uuid,
	pub hash: String,
	pub salt: Option<String>,
	pub created_at: DateTime<Utc>,
}
