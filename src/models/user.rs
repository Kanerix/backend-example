//! Users database models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User role model
#[derive(sqlx::Type, Serialize, Deserialize, Debug, Clone)]
#[sqlx(type_name = "role", rename_all = "lowercase")]
pub enum UserRole {
	ADMIN,
	MODERATOR,
	USER,
}

/// User model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
	pub id: Uuid,
	pub username: String,
	pub email: String,
	pub role: UserRole,
	pub created_at: DateTime<Utc>,
}

/// User password model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Password {
	pub id: Uuid,
	pub hash: String,
	pub salt: Option<String>,
	pub created_at: DateTime<Utc>,
}
