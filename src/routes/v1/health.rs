use aide::transform::TransformOperation;

use crate::error::HandlerResult;

#[axum::debug_handler]
pub async fn handler() -> HandlerResult<()> {
	Ok(())
}

pub fn docs(op: TransformOperation) -> TransformOperation {
	op.description("Check health").tag("health")
}
