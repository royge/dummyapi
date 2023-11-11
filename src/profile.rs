use super::handlers;
use super::auth;
use super::models::profile::{Profile};
use std::convert::Infallible;
use warp::Filter;
use super::store::Db;

pub fn profiles(db: Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    create(db.clone()).or(get(db))
}

pub fn create(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("profiles")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::profile::create)
}

pub fn get(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("profiles" / u8)
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(auth::with_auth(db))
        .and_then(handlers::profile::get)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (Profile,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
