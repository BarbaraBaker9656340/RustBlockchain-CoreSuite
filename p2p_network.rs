use std::collections::HashSet;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[derive(Debug, Clone)]
pub struct P2PNetwork {
    pub node_addr: SocketAddr,
    pub peers: HashSet<SocketAddr>,
}

impl P2PNetwork {
    pub fn new(addr: SocketAddr) -> Self {
        P2PNetwork {
            node_addr: addr,
            peers: HashSet::new(),
        }
    }

    pub async fn start_server(&self) {
        let listener = TcpListener::bind(&self.node_addr).await.unwrap();
        println!("P2P节点启动: {}", self.node_addr);
        
        while let Ok((stream, addr)) = listener.accept().await {
            self.handle_connection(stream, addr).await;
        }
    }

    async fn handle_connection(&self, mut stream: TcpStream, addr: SocketAddr) {
        let mut buffer = [0; 1024];
        if stream.read(&mut buffer).await.is_ok() {
            let msg = String::from_utf8_lossy(&buffer).trim().to_string();
            println!("收到来自 {} 的消息: {}", addr, msg);
            self.peers.insert(addr);
        }
    }

    pub async fn broadcast(&self, message: &str) {
        for peer in &self.peers {
            if let Ok(mut stream) = TcpStream::connect(peer).await {
                let _ = stream.write_all(message.as_bytes()).await;
            }
        }
    }
}
