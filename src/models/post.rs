use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Post {
	pub id: Uuid,
	pub title: String,
	pub body: String,
	pub created_at: DateTime<Utc>,
}

pub struct Comment {
	pub id: Uuid,
	pub user_id: Uuid,
	pub post_id: Uuid,
	pub body: String,
	pub created_at: DateTime<Utc>,
}
