use crate::error::HandlerResult;

#[axum::debug_handler]
pub async fn failure() -> HandlerResult<()> {
	"abc".parse::<i32>()?;
	Ok(())
}
