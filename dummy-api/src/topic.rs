use super::auth;
use super::handlers;
use super::models::topic::{Topic};
use std::convert::Infallible;
use warp::Filter;
use super::store::Db;

pub fn topics(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    create(db.clone()).or(update(db))
}

pub fn create(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("topics")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db.clone()))
        .and(auth::with_auth(db.clone()))
        .and_then(handlers::topic::create)
}

pub fn update(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("topics" / u8)
        .and(warp::put())
        .and(json_body())
        .and(with_db(db.clone()))
        .and(auth::with_auth(db.clone()))
        .and_then(handlers::topic::update)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (Topic,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
