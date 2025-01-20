//! All endpoints related to posts.

pub mod comments;
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
		.api_route("/create", post(create))
		.api_route("/delete/{id}", delete(destroy))
		.api_route("/edit/{id}", put(edit))
		.api_route("/posts", get(list))
		.nest("/comments", comments::routes())
}

/// Request body for the create/edit posts endpoint.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct PostRequest {
	title: String,
	body: String,
}

/// Parameters to identify a post.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct PostParams {
	post_id: Uuid,
}
