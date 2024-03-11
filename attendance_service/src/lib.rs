use bcrypt::{hash, verify};
use dotenv::dotenv;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::env;
use warp::{http::StatusCode, reject, Filter, Rejection, Reply};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub token: String,
}

pub async fn hash_password(password: &str) -> String {
    hash(password, 4).unwrap()
}

pub async fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap()
}

pub async fn generate_token(username: &str) -> String {
    dotenv().ok();
    let secret = env::var("SECRET_KEY").expect("SECRET_KEY must be set");

    let expiration = chrono::Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: username.to_owned(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .unwrap()
}

pub async fn login(user: LoginRequest) -> Result<impl Reply, Rejection> {
    // Here, you should verify the username and password against your user store
    // This example simply hashes the provided password and compares it for demonstration.
    // In a real scenario, you would look up the user in the database and verify the password hash.

    // Placeholder for password verification logic
    let password_hash = hash_password(&user.password).await;
    if verify_password(&user.password, &password_hash).await {
        let token = generate_token(&user.username).await;
        Ok(warp::reply::json(&LoginResponse { token }))
    } else {
        Err(reject::custom(MyError::InvalidCredentials))
    }
}

// Define more functions for handling user registration, token verification, etc.
