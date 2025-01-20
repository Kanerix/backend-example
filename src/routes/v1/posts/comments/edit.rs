use aide::axum::IntoApiResponse;

use crate::error::HandlerResult;

#[axum::debug_handler]
pub async fn edit() -> HandlerResult<impl IntoApiResponse> {
	Ok(())
}
