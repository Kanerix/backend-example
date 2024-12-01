use axum::response::IntoResponse;

use crate::error::HandlerResult;

pub async fn list() -> HandlerResult<impl IntoResponse> {
    Ok(())
}
