//! All endpoints related to posts.

pub mod create;
pub mod delete;
pub mod edit;
pub mod list;

use aide::axum::{
	routing::{get_with, put_with},
	ApiRouter,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::AppState;

pub fn routes(state: AppState) -> ApiRouter {
	ApiRouter::new()
	    .api_route(
			"/",
			get_with(list::handler, list::docs).post_with(create::handler, create::docs)
		)
		.api_route(
			"/{comment_id}",
			put_with(edit::handler, edit::docs).delete_with(delete::handler, delete::docs),
		)
		.with_state(state)
}

/// Request body for the create/edit comments endpoint.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CommentRequest {
	body: String,
}

/// Parameters to identify a comment.
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct CommentParams {
	post_id: Uuid,
	comment_id: Uuid,
}
