use std::net::SocketAddr;
use tokio::sync::mpsc::Sender;

pub async fn run(tx: Sender<String>) {
    let api = employee_management_service::api();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3031)); // Use a different port than the auth service
    warp::serve(api).run(addr).await;

    let _ = tx.send("Employee Management Service stopped".into()).await;
} // This function should match whatever setup function you have in your `employee_management_service` crate
pub async fn start_employee_management_service() {
    let api = employee_management_service::api(); // This function should return your warp filter setup

    warp::serve(api)
        .run(([127, 0, 0, 1], 3031)) // Note: Different port than auth service
        .await;
}

// This is a placeholder for your actual runner function
