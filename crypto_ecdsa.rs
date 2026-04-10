use k256::{ecdsa::{SigningKey, VerifyingKey, Signature, signature::Signer, signature::Verifier}, EncodedPoint};
use rand::rngs::OsRng;
use hex;

pub struct ECDSAHandler;

impl ECDSAHandler {
    pub fn generate_key_pair() -> (SigningKey, VerifyingKey) {
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = VerifyingKey::from(&signing_key);
        (signing_key, verifying_key)
    }

    pub fn sign_message(signing_key: &SigningKey, message: &[u8]) -> String {
        let signature: Signature = signing_key.sign(message);
        hex::encode(signature.to_bytes())
    }

    pub fn verify_signature(verifying_key: &VerifyingKey, message: &[u8], sig_hex: &str) -> bool {
        match hex::decode(sig_hex) {
            Ok(sig_bytes) => {
                match Signature::from_slice(&sig_bytes) {
                    Ok(sig) => verifying_key.verify(message, &sig).is_ok(),
                    Err(_) => false,
                }
            }
            Err(_) => false,
        }
    }

    pub fn pubkey_to_address(verifying_key: &VerifyingKey) -> String {
        let point = verifying_key.to_encoded_point(false);
        let hash = sha3::Keccak256::digest(&point.as_bytes()[1..]);
        format!("0x{}", hex::encode(&hash[12..]))
    }
}
