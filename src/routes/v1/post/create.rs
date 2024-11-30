use axum::response::IntoResponse;

use crate::error::HandlerResult;

pub async fn create() -> HandlerResult<impl IntoResponse> {
    Ok(())
}
