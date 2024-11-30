use axum::response::IntoResponse;

use crate::error::HandlerResult;

pub async fn posts() -> HandlerResult<impl IntoResponse> {
    Ok(())
}
