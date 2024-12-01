use axum::response::IntoResponse;

use crate::error::HandlerResult;

pub async fn comment() -> HandlerResult<impl IntoResponse> {
	Ok(())
}
