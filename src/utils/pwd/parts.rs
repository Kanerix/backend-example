use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

use super::{error::Error, DEFAULT_SCHEME};

/// What parts a password needs to be hashed.
///
/// This is needed when turning a password into a hash.
pub struct PwdParts {
	pub scheme_name: String,
	pub salt: String,
	pub pwd: String,
}

/// What passwords get turned into when hashed.
///
/// This is needed for validating a password hash.
#[derive(Debug)]
pub struct HashParts {
	pub scheme_name: String,
	pub hash: String,
}

impl PwdParts {
	/// Creates a new [`PwdParts`] structure.
	///
	/// This will have the latest scheme for hashing.
	pub fn new(pwd: String, salt: String) -> Self {
		Self {
			scheme_name: DEFAULT_SCHEME.into(),
			salt,
			pwd,
		}
	}
}

impl HashParts {
	/// Creates a new [`HashParts`] structure.
	pub fn new(scheme_name: String, hash: String) -> Self {
		Self { scheme_name, hash }
	}
}

lazy_static! {
	static ref PWD_PARTS_REGEX: Regex = Regex::new(r"^#(?<scheme_name>\w+)#(?<hash>.+)$").unwrap();
}

impl FromStr for HashParts {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let captures = PWD_PARTS_REGEX.captures(s).ok_or(Error::PwdParsingFailed(
			"password hash is not in the correct format".to_string(),
		))?;

		let scheme_name = captures
			.name("scheme_name")
			.ok_or(Error::PwdParsingFailed(
				"missing \"scheme_name\" part in password hash".to_string(),
			))?
			.as_str()
			.to_string();
		let hash = captures
			.name("hash")
			.ok_or(Error::PwdParsingFailed(
				"missing \"hash\" part in password hash".to_string(),
			))?
			.as_str()
			.to_string();

		Ok(HashParts::new(scheme_name, hash))
	}
}
