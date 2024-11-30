mod comment;
mod create;
mod delete;
mod edit;
mod posts;

use axum::{
	routing::{delete, get, patch, post},
	Router,
};
use sqlx::PgPool;

pub use comment::comment;
pub use create::create;
pub use delete::destroy;
pub use edit::edit;
pub use posts::posts;

pub fn routes() -> Router<PgPool> {
	Router::new()
		.route("/comment/:id", post(comment))
		.route("/create", post(create))
		.route("/delete/:id", delete(destroy))
		.route("/edit/:id", patch(edit))
		.route("posts", get(posts))
}
