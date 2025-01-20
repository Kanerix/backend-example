//! All endpoints related to posts.

pub mod create;
pub mod delete;
pub mod edit;
pub mod list;

use aide::axum::{
	routing::{delete, get, post, put},
	ApiRouter,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

pub use create::create;
pub use delete::destroy;
pub use edit::edit;
pub use list::list;

pub fn routes() -> ApiRouter<PgPool> {
	ApiRouter::new()
		.api_route("/{id}/create", post(create))
		.api_route("/{id}/delete", delete(destroy))
		.api_route("/{id}/edit", put(edit))
		.api_route("/comments", get(list))
}

/// Request body for the create/edit comments endpoint.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PostRequest {
	title: String,
	body: String,
}

/// Parameters to identify a comment.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CommentParams {
	post_id: Uuid,
	comment_id: Uuid,
}
