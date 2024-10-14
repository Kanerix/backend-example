use crate::error::HandlerResult;

#[utoipa::path(
    get,
    path = "/api/v1/health",
    responses(
        (status = 200, description = "Successful health check"),
    ),
)]
pub async fn health() -> HandlerResult<()> {
	Ok(())
}
