use hyper::{Body, Request, Response, Server};
use routerify::Router;
use super::blockchain_core::Blockchain;
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn start_rpc_server(port: u16, chain: Arc<Mutex<Blockchain>>) {
    let router = Router::builder()
        .get("/chain/height", |_req| async move {
            let chain = chain.lock().await;
            Ok(Response::new(Body::from(format!("{}", chain.chain.len()))))
        })
        .build()
        .unwrap();

    let addr = ([0, 0, 0, 0], port).into();
    let server = Server::bind(&addr).serve(router.into_make_svc());

    println!("RPC服务启动: {}", addr);
    let _ = server.await;
}
