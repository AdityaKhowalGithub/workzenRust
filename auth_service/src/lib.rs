use bcrypt::{hash, verify};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

pub async fn hash_password(password: &str) -> String {
    hash(password, 4).unwrap()
}

pub async fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap()
}

pub async fn generate_token(user_id: &str) -> String {
    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
    .unwrap()
}

// Define more functions for handling login, logout, and token verification.
// auth_service/src/lib.rs

pub fn api() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Define your authentication routes here
    warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .map(|login_request| {
            // Handle login request
            warp::reply::json(&"Token placeholder")
        })
}
