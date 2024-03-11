// auth_service/src/main.rs
use warp::Filter;

#[tokio::main]
async fn main() {
    let api = auth_service::routes(); // Define this function in lib.rs to encapsulate your routes.

    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;
}
