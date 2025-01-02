use crate::error::HandlerResult;

use super::HEALTH_TAG;

#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Successful health check"),
    ),
    tag = HEALTH_TAG
)]
#[axum::debug_handler]
pub async fn health() -> HandlerResult<()> {
	Ok(())
}
