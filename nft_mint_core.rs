use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct NFTMetadata {
    pub token_id: String,
    pub name: String,
    pub creator: String,
    pub owner: String,
    pub uri: String,
    pub mint_time: u64,
}

pub struct NFTMintCore {
    nfts: HashMap<String, NFTMetadata>,
    owner_map: HashMap<String, Vec<String>>,
}

impl NFTMintCore {
    pub fn new() -> Self {
        NFTMintCore {
            nfts: HashMap::new(),
            owner_map: HashMap::new(),
        }
    }

    pub fn mint_nft(&mut self, metadata: NFTMetadata) -> bool {
        if self.nfts.contains_key(&metadata.token_id) {
            return false;
        }
        self.owner_map.entry(metadata.owner.clone()).or_default().push(metadata.token_id.clone());
        self.nfts.insert(metadata.token_id.clone(), metadata);
        true
    }

    pub fn transfer_nft(&mut self, token_id: &str, from: &str, to: &str) -> bool {
        if let Some(nft) = self.nfts.get_mut(token_id) {
            if nft.owner != from {
                return false;
            }
            nft.owner = to.to_string();
            if let Some(list) = self.owner_map.get_mut(from) {
                list.retain(|id| id != token_id);
            }
            self.owner_map.entry(to.to_string()).or_default().push(token_id.to_string());
            return true;
        }
        false
    }
}
