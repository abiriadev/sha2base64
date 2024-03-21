use base64ct::{Base64, Base64UrlUnpadded, Encoding};
use sha2::{Digest, Sha256};

fn main() {
	let hash = Sha256::digest("hello world");
	let result = Base64UrlUnpadded::encode_string(&hash);

	println!("{}", &result[..16]);
}
