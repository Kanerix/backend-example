use std::{fs::File, io::Read, path::Path, sync::OnceLock};

use jsonwebtoken::{DecodingKey, EncodingKey};

pub(crate) fn jwt_decode_key() -> &'static DecodingKey {
	static JWT_DECODE_KEY: OnceLock<DecodingKey> = OnceLock::new();
	JWT_DECODE_KEY.get_or_init(|| {
		let path = Path::new("./keys/ed25519_public.pem");
		let file = File::open(path).unwrap();

		let mut reader = std::io::BufReader::new(&file);
		let mut bytes = Vec::new();

		reader.read_to_end(&mut bytes).unwrap();
		DecodingKey::from_ed_pem(&bytes).unwrap()
	})
}

pub(crate) fn jwt_encode_key() -> &'static EncodingKey {
	static JWT_ENCODE_KEY: OnceLock<EncodingKey> = OnceLock::new();
	JWT_ENCODE_KEY.get_or_init(|| {
		let path = Path::new("./keys/ed25519_private.pem");
		let file = File::open(path).unwrap();

		let mut reader = std::io::BufReader::new(&file);
		let mut bytes = Vec::new();

		reader.read_to_end(&mut bytes).unwrap();
		EncodingKey::from_ed_pem(&bytes).unwrap()
	})
}
