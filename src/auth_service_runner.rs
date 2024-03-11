// use warp::Filter;
use std::net::SocketAddr;
use tokio::sync::mpsc::Sender;

// This function should match whatever setup function you have in your `auth_service` crate
pub async fn start_auth_service() {
    let api = auth_service::api(); // This function should return your warp filter setup

    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
}

// src/auth_service_runner.rs

pub async fn run(tx: Sender<String>) {
    let api = auth_service::api();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    warp::serve(api).run(addr).await;

    let _ = tx.send("Authentication Service stopped".into()).await;
}
