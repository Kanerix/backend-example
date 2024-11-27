//! Errors that can occur when handling different schemes.

use argon2::password_hash::Error as Argon2Error;

/// A type alias for [`Result<T, Error>`].
///
/// Used by this module to return the same error for each [`Result`].
pub type Result<T> = std::result::Result<T, Error>;

/// All the different errors the `scheme` module might produce.
#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("no scheme named \"{0}\" exist")]
	SchemeNotFound(String),
	#[error("error hashing password: {0}")]
	PwdHash(#[from] Argon2Error),
}
