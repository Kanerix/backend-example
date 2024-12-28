//! All endpoints related to posts.

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
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

pub use create::create;
pub use delete::destroy;
pub use edit::edit;
pub use list::list;

pub fn routes() -> Router<PgPool> {
	Router::new()
		.route("/:id/create", post(create))
		.route("/:id/delete", delete(destroy))
		.route("/:id/edit", patch(edit))
		.route("/comments", get(list))
}

/// Request body for the create/edit comments endpoint.
#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct PostRequest {
	title: String,
	body: String,
}

/// Parameters to identify a comment.
#[derive(Debug, Deserialize, Serialize)]
pub struct CommentParams {
	post_id: Uuid,
	comment_id: Uuid,
}
