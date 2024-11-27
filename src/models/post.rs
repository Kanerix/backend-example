use uuid::Uuid;

pub struct Post {
	pub id: Uuid,
	pub title: String,
	pub content: String,
}
