/// A type alias for [`Result<T, Error>`].
///
/// Used by this module to return the same error for each [`Result`].
pub type Result<T> = std::result::Result<T, Error>;

/// All the different errors the `pwd` module might produce.
#[non_exhaustive]
#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error(transparent)]
	TokenError(#[from] jsonwebtoken::errors::Error),
}
