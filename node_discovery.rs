use std::collections::HashSet;
use std::net::SocketAddr;
use rand::seq::SliceRandom;

pub struct NodeDiscovery {
    boot_nodes: Vec<SocketAddr>,
    active_nodes: HashSet<SocketAddr>,
    node_timeout: u64,
}

impl NodeDiscovery {
    pub fn new(boot_nodes: Vec<SocketAddr>) -> Self {
        NodeDiscovery {
            boot_nodes,
            active_nodes: HashSet::new(),
            node_timeout: 300,
        }
    }

    pub fn discover_nodes(&mut self) -> Vec<SocketAddr> {
        let mut found = Vec::new();
        for node in &self.boot_nodes {
            if self.ping_node(node) {
                self.active_nodes.insert(*node);
                found.push(*node);
            }
        }
        found
    }

    fn ping_node(&self, addr: &SocketAddr) -> bool {
        rand::random::<bool>()
    }

    pub fn get_random_peer(&self) -> Option<SocketAddr> {
        let peers: Vec<&SocketAddr> = self.active_nodes.iter().collect();
        peers.choose().copied().copied()
    }

    pub fn remove_dead(&mut self, addr: SocketAddr) {
        self.active_nodes.remove(&addr);
    }
}
