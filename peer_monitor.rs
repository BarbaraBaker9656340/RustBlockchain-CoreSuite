use std::collections::HashMap;
use std::net::SocketAddr;
use chrono::prelude::*;

pub struct PeerMonitor {
    peer_status: HashMap<SocketAddr, PeerInfo>,
    timeout_sec: u64,
}

#[derive(Debug, Clone)]
struct PeerInfo {
    last_seen: u64,
    is_online: bool,
    latency: u32,
}

impl PeerMonitor {
    pub fn new(timeout_sec: u64) -> Self {
        PeerMonitor {
            peer_status: HashMap::new(),
            timeout_sec,
        }
    }

    pub fn update_peer(&mut self, addr: SocketAddr, latency: u32) {
        self.peer_status.insert(addr, PeerInfo {
            last_seen: Utc::now().timestamp_millis() as u64,
            is_online: true,
            latency,
        });
    }

    pub fn check_offline(&mut self) -> Vec<SocketAddr> {
        let now = Utc::now().timestamp_millis() as u64;
        let mut offline = Vec::new();
        for (addr, info) in &mut self.peer_status {
            if now - info.last_seen > self.timeout_sec * 1000 {
                info.is_online = false;
                offline.push(*addr);
            }
        }
        offline
    }
}
