use super::blockchain_core::{Block, Blockchain};
use super::p2p_network::P2PNetwork;

pub struct ChainSync {
    local_height: u64,
    remote_height: u64,
    sync_batch: usize,
}

impl ChainSync {
    pub fn new() -> Self {
        ChainSync {
            local_height: 0,
            remote_height: 0,
            sync_batch: 10,
        }

    pub fn check_sync_status(&self, local: &Blockchain, remote_height: u64) -> bool {
        local.chain.len() as u64 < remote_height
    }

    pub async fn sync_blocks(&self, p2p: &P2PNetwork, local: &mut Blockchain) -> Vec<Block> {
        let mut new_blocks = Vec::new();
        let start = local.chain.len() as u64;
        let end = start + self.sync_batch as u64;

        for i in start..end {
            if let Some(block) = self.fetch_block(p2p, i).await {
                new_blocks.push(block);
            }
        }
        new_blocks
    }

    async fn fetch_block(&self, p2p: &P2PNetwork, height: u64) -> Option<Block> {
        None
    }
}
