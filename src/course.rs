use super::auth;
use super::handlers;
use super::models::course::{self, Course};
use super::models::ListOptions;
use std::convert::Infallible;
use warp::Filter;
use super::store::Db;

pub fn courses(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    create(db.clone())
        .or(get(db.clone()))
        .or(update(db.clone()))
        .or(list(db))
}

pub fn create(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("courses")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db.clone()))
        .and(auth::with_auth(db.clone()))
        .and_then(handlers::course::create)
}

pub fn get(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("courses" / u8)
        .and(warp::get())
        .and(with_db(db.clone()))
        .and(auth::with_auth(db.clone()))
        .and_then(handlers::course::get)
}

pub fn update(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("courses" / u8)
        .and(warp::put())
        .and(json_body())
        .and(with_db(db.clone()))
        .and(auth::with_auth(db.clone()))
        .and_then(handlers::course::update)
}

pub fn list(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("courses")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and(with_db(db.clone()))
        .and(auth::with_auth(db.clone()))
        .and_then(handlers::course::list)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (Course,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub async fn find<'a>(id: u8, db: &'a Db) -> Result<Course, Box<dyn std::error::Error>> {
    if id == 0 {
        return Err("Course ID is required!".into());
    }

    let db = db.lock().await;

    let docs: &Vec<Vec<u8>> = db.get(course::COURSES).unwrap();
    for data in docs.iter() {
        let course: Course = bincode::deserialize(&data).unwrap();
        if course.id == id {
            return Ok(course);
        }
    }

    Err("invalid course".into())
}
