use std::collections::HashMap;
use sha2::{Sha256, Digest};

pub struct DecentralStore {
    nodes: Vec<String>,
    file_map: HashMap<String, Vec<u8>>,
    replica: usize,
}

impl DecentralStore {
    pub fn new(replica: usize) -> Self {
        DecentralStore {
            nodes: Vec::new(),
            file_map: HashMap::new(),
            replica,
        }
    }

    pub fn add_node(&mut self, node_id: String) {
        self.nodes.push(node_id);
    }

    pub fn store_file(&mut self, data: Vec<u8>) -> String {
        let cid = hex::encode(Sha256::digest(&data));
        self.file_map.insert(cid.clone(), data);
        cid
    }

    pub fn get_file(&self, cid: &str) -> Option<Vec<u8>> {
        self.file_map.get(cid).cloned()
    }

    pub fn replicate(&self) -> bool {
        self.nodes.len() >= self.replica
    }
}
