use aide::axum::IntoApiResponse;

use crate::error::HandlerResult;

#[axum::debug_handler]
pub async fn list() -> HandlerResult<impl IntoApiResponse> {
	Ok(())
}
