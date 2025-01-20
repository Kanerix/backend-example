use aide::{axum::IntoApiResponse, transform::TransformOperation};

use crate::error::HandlerResult;

#[axum::debug_handler]
pub async fn handler() -> HandlerResult<impl IntoApiResponse> {
	Ok(())
}

pub fn docs(op: TransformOperation) -> TransformOperation {
	op.description("Create a comment.").tag("posts")
}
