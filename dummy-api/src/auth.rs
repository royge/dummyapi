use super::config::CONFIG;
use super::handlers;
use super::models::profile::{Credentials, Db};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_derive::Serialize;
use std::convert::Infallible;
use warp::Filter;
use rand::Rng;

pub fn auth(db: Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    login(db)
}

pub fn login(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("auth")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::auth::login)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (Credentials,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[derive(Serialize)]
pub struct Claims {
    user_id: u8,
    exp: i64,
}

pub fn generate_token(user_id: u8) -> Result<String, Box<dyn std::error::Error>> {
    let expiration = Utc::now() + Duration::hours(1);
    let claims = Claims {
        user_id,
        exp: expiration.timestamp(),
    };

    let config = CONFIG
        .get()
        .expect("Application is not properly configured.");

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret.as_ref()),
    )?;
    Ok(token)
}

pub fn generate_secret_key(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let key: Vec<u8> = (0..length).map(|_| rng.gen()).collect();
    key
}
