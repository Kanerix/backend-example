pub mod claims;
pub mod error;
mod keys;

use claims::{TokenClaims, TokenUser};
use error::{Error, Result};
use jsonwebtoken::{decode, encode, TokenData};
use rand::{distributions::Alphanumeric, thread_rng, Rng};

pub fn generate_access_token(user: impl Into<TokenUser>) -> Result<String> {
	encode(
		&jsonwebtoken::Header {
			alg: jsonwebtoken::Algorithm::EdDSA,
			..Default::default()
		},
		&TokenClaims::new(user),
		&keys::jwt_encode_key(),
	)
	.map_err(Error::TokenError)
}

pub fn decode_access_token(token: &str) -> Result<TokenData<TokenClaims>> {
	decode(
		token,
		&keys::jwt_decode_key(),
		&jsonwebtoken::Validation::default(),
	)
	.map_err(Error::TokenError)
}

pub fn generate_refresh_token() -> String {
	let rng = thread_rng();
	rng.sample_iter(&Alphanumeric)
		.take(32)
		.map(char::from)
		.collect()
}
