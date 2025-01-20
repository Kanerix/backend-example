use crate::error::HandlerResult;

#[axum::debug_handler]
pub async fn health() -> HandlerResult<()> {
	Ok(())
}
