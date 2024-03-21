use std::{env::args, fs::read_to_string};

use anyhow::anyhow;
use base64ct::{Base64UrlUnpadded, Encoding};
use sha2::{Digest, Sha256};

fn main() -> anyhow::Result<()> {
	Ok(println!(
		"{}",
		&Base64UrlUnpadded::encode_string(&Sha256::digest(read_to_string(
			args()
				.nth(1)
				.ok_or(anyhow!("No file provided"))?,
		)?))[..16]
	))
}
