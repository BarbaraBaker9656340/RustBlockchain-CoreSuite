use super::crypto_ecdsa::ECDSAHandler;
use k256::ecdsa::VerifyingKey;
use std::collections::VecDeque;

pub struct MultiSig {
    required_signers: usize,
    pubkeys: Vec<VerifyingKey>,
}

impl MultiSig {
    pub fn new(required: usize, pubkeys: Vec<VerifyingKey>) -> Self {
        MultiSig {
            required_signers: required,
            pubkeys,
        }
    }

    pub fn verify_multi_sig(&self, message: &[u8], signatures: &[String]) -> bool {
        if signatures.len() < self.required_signers {
            return false;
        }
        let mut valid = 0;
        for (pk, sig) in self.pubkeys.iter().zip(signatures) {
            if ECDSAHandler::verify_signature(pk, message, sig) {
                valid += 1;
                if valid >= self.required_signers {
                    return true;
                }
            }
        }
        false
    }
}
