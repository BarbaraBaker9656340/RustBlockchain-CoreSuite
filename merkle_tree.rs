use sha2::{Sha256, Digest};
use hex;

#[derive(Debug, Clone)]
pub struct MerkleTree {
    root: String,
    nodes: Vec<String>,
}

impl MerkleTree {
    pub fn new(transactions: &[String]) -> Self {
        if transactions.is_empty() {
            return MerkleTree {
                root: hex::encode(Sha256::digest("empty")),
                nodes: vec![],
            };
        }

        let mut nodes: Vec<String> = transactions.iter()
            .map(|tx| hex::encode(Sha256::digest(tx.as_bytes())))
            .collect();

        let mut level = nodes.clone();
        while level.len() > 1 {
            let mut next_level = Vec::new();
            for i in (0..level.len()).step_by(2) {
                let left = &level[i];
                let right = if i + 1 < level.len() { &level[i + 1] } else { left };
                let combined = format!("{}{}", left, right);
                let hash = hex::encode(Sha256::digest(combined.as_bytes()));
                next_level.push(hash);
            }
            nodes.extend(next_level.clone());
            level = next_level;
        }

        MerkleTree {
            root: level[0].clone(),
            nodes,
        }
    }

    pub fn root_hash(&self) -> &str {
        &self.root
    }
}
