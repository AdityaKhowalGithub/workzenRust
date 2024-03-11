use std::{env, thread};
use tokio::sync::mpsc;

mod auth_service_runner;
mod employee_management_service_runner;
// Import other service runners as modules

#[tokio::main]
async fn main() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    tokio::spawn(async move {
        auth_service_runner::run(tx.clone()).await;
    });

    tokio::spawn(async move {
        employee_management_service_runner::run(tx).await;
    });

    // Example of how to handle messages from services (this will run indefinitely for demonstration)
    while let Some(message) = rx.recv().await {
        println!("{}", message);
    }
}

// In `auth_service_runner.rs` and `employee_management_service_runner.rs`, define functions to start each service
