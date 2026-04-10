use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::Rng;
use hex;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

pub struct P2PEncryptor {
    key: [u8; 32],
}

impl P2PEncryptor {
    pub fn new(key: [u8; 32]) -> Self {
        P2PEncryptor { key }
    }

    pub fn generate_key() -> [u8; 32] {
        rand::random()
    }

    pub fn encrypt(&self, data: &[u8]) -> String {
        let iv = rand::random::<[u8; 16]>();
        let cipher = Aes256Cbc::new_from_slices(&self.key, &iv).unwrap();
        let ciphertext = cipher.encrypt_vec(data);
        let mut result = iv.to_vec();
        result.extend(ciphertext);
        hex::encode(result)
    }

    pub fn decrypt(&self, encrypted_hex: &str) -> Option<Vec<u8>> {
        let data = hex::decode(encrypted_hex).ok()?;
        let (iv, ciphertext) = data.split_at(16);
        let cipher = Aes256Cbc::new_from_slices(&self.key, iv).ok()?;
        cipher.decrypt_vec(ciphertext).ok()
    }
}
