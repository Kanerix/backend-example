use regex::Regex;
use std::{str::FromStr, sync::OnceLock};

use super::{error::Error, DEFAULT_SCHEME};

/// What parts a password needs to be hashed.
///
/// This is needed when turning a password into a hash.
pub struct PwdParts {
	pub scheme: String,
	pub salt: String,
	pub pwd: String,
}

/// What passwords get turned into when hashed.
///
/// This is needed for validating a password hash.
#[derive(Debug, Clone)]
pub struct HashParts {
	pub scheme: String,
	pub hash: String,
}

impl PwdParts {
	/// Creates a new [`PwdParts`] structure.
	///
	/// This will have the latest scheme for hashing.
	pub fn new(pwd: String, salt: String) -> Self {
		Self {
			scheme: DEFAULT_SCHEME.into(),
			salt,
			pwd,
		}
	}
}

impl HashParts {
	/// Creates a new [`HashParts`] structure.
	pub fn new(scheme: String, hash: String) -> Self {
		Self { scheme, hash }
	}
}

fn pwd_parts_regex() -> &'static Regex {
	static PWD_PARTS_REGEX: OnceLock<Regex> = OnceLock::new();
	PWD_PARTS_REGEX.get_or_init(|| Regex::new(r"^#(?<scheme>\w+)#(?<hash>.+)$").unwrap())
}

impl FromStr for HashParts {
	type Err = Error;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		let captures = pwd_parts_regex()
			.captures(s)
			.ok_or(Error::PwdParsingFailed(
				"password hash is not in the correct format".to_string(),
			))?;

		let get_group = |name: &str| -> Result<String, Self::Err> {
			Ok(captures
				.name(name)
				.ok_or(Error::PwdParsingFailed(
					"missing \"scheme\" part in password hash".to_string(),
				))?
				.as_str()
				.to_string())
		};

		let scheme = get_group("scheme")?;
		let hash = get_group("hash")?;

		Ok(HashParts::new(scheme, hash))
	}
}
