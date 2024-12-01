use axum::response::IntoResponse;

use crate::error::HandlerResult;

pub async fn edit() -> HandlerResult<impl IntoResponse> {
    Ok(())
}
