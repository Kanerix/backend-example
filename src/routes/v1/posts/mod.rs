pub mod comment;
pub mod create;
pub mod delete;
pub mod edit;
pub mod list;

use axum::{
	routing::{delete, get, patch, post},
	Router,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

pub use comment::comment;
pub use create::create;
pub use delete::destroy;
pub use edit::edit;
pub use list::list;
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

pub fn routes() -> Router<PgPool> {
	Router::new()
		.route("/comment/:id", post(comment))
		.route("/create", post(create))
		.route("/delete/:id", delete(destroy))
		.route("/edit/:id", patch(edit))
		.route("/posts", get(list))
}

/// Request body for the create/edit post endpoint.
#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct PostRequest {
	title: String,
	body: String,
}

/// Parameters that some endpoints use to identify a post.
#[derive(Debug, Deserialize, Serialize)]
pub struct PostParams {
	post_id: Uuid,
}
