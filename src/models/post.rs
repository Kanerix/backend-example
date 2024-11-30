//! Posts database models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Post model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Post {
	pub id: Uuid,
	pub title: String,
	pub body: String,
	pub created_at: DateTime<Utc>,
}

/// Post comment model
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Comment {
	pub id: Uuid,
	pub user_id: Uuid,
	pub post_id: Uuid,
	pub body: String,
	pub created_at: DateTime<Utc>,
}
