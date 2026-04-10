use super::crypto_ecdsa::ECDSAHandler;
use k256::{ecdsa::SigningKey, ecdsa::VerifyingKey};
use hex;

pub struct Wallet {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
    address: String,
}

impl Wallet {
    pub fn new() -> Self {
        let (sk, vk) = ECDSAHandler::generate_key_pair();
        let address = ECDSAHandler::pubkey_to_address(&vk);
        Wallet {
            signing_key: sk,
            verifying_key: vk,
            address,
        }
    }

    pub fn from_private_key(sk_hex: &str) -> Option<Self> {
        let bytes = hex::decode(sk_hex).ok()?;
        let sk = SigningKey::from_slice(&bytes).ok()?;
        let vk = VerifyingKey::from(&sk);
        let address = ECDSAHandler::pubkey_to_address(&vk);
        Some(Wallet {
            signing_key: sk,
            verifying_key: vk,
            address,
        })
    }

    pub fn sign(&self, message: &[u8]) -> String {
        ECDSAHandler::sign_message(&self.signing_key, message)
    }

    pub fn address(&self) -> &str {
        &self.address
    }

    pub fn public_key_hex(&self) -> String {
        hex::encode(self.verifying_key.to_encoded_point(false).as_bytes())
    }
}
