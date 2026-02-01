use bip39::{Language, Mnemonic,};
use k256::{ecdsa::SigningKey};
use sha3::{Digest, Keccak256};
use hex::encode;


pub fn generate_wallet() -> (String, String, String) {
    let mnemonic = Mnemonic::generate_in(Language::English, 24).unwrap();
    let seed = mnemonic.to_seed("");
    let private_key_bytes= &seed[0..32];

    let signing_key = SigningKey::from_bytes(private_key_bytes.into()).unwrap();
    let verifying_key = signing_key.verifying_key();
    let verfify_key_point = verifying_key.to_encoded_point(false);
    let public_key_bytes = &verfify_key_point.as_bytes()[1..];
    let hash = Keccak256::digest(public_key_bytes);
    let address = &hash[12..];
    (
        mnemonic.to_string(),
        encode(private_key_bytes),
        format!("0x{}", encode(address)),
    )
}

