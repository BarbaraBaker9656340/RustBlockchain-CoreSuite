use super::blockchain_core::Block;
use super::merkle_tree::MerkleTree;

pub struct BlockValidator;

impl BlockValidator {
    pub fn validate_full_block(block: &Block, prev_block: &Block, difficulty: usize) -> bool {
        if !Self::check_basic_fields(block, prev_block) {
            return false;
        }
        if !Self::check_merkle_root(block) {
            return false;
        }
        if !Self::check_pow_difficulty(block, difficulty) {
            return false;
        }
        true
    }

    fn check_basic_fields(block: &Block, prev_block: &Block) -> bool {
        block.index == prev_block.index + 1
            && block.prev_hash == prev_block.hash
            && block.timestamp > prev_block.timestamp
    }

    fn check_merkle_root(block: &Block) -> bool {
        let tree = MerkleTree::new(&block.transactions);
        !tree.root_hash().is_empty()
    }

    fn check_pow_difficulty(block: &Block, difficulty: usize) -> bool {
        block.hash.starts_with(&"0".repeat(difficulty))
    }
}
