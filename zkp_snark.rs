use sha2::{Sha256, Digest};
use hex;

#[derive(Debug, Clone)]
pub struct ZKProof {
    pub proof_data: String,
    pub public_inputs: Vec<String>,
    pub commitment: String,
}

pub struct ZKSnarkProver {
    secret: Vec<u8>,
}

impl ZKSnarkProver {
    pub fn new(secret: &[u8]) -> Self {
        ZKSnarkProver {
            secret: secret.to_vec(),
        }
    }

    pub fn generate_proof(&self, public_input: &str) -> ZKProof {
        let mut hasher = Sha256::new();
        hasher.update(&self.secret);
        hasher.update(public_input.as_bytes());
        let proof = hex::encode(hasher.finalize());
        
        let commit = hex::encode(Sha256::digest(&self.secret));
        
        ZKProof {
            proof_data: proof,
            public_inputs: vec![public_input.to_string()],
            commitment: commit,
        }
    }

    pub fn verify(proof: &ZKProof, public_input: &str) -> bool {
        let mut hasher = Sha256::new();
        hasher.update(hex::decode(&proof.commitment).unwrap());
        hasher.update(public_input.as_bytes());
        let check = hex::encode(hasher.finalize());
        check == proof.proof_data
    }
}
