use aide::transform::TransformOperation;

use crate::error::HandlerResult;

#[axum::debug_handler]
pub async fn handler() -> HandlerResult<()> {
	"abc".parse::<i32>()?;
	Ok(())
}

pub fn docs(op: TransformOperation) -> TransformOperation {
	op.description("Force failure").tag("health")
}
