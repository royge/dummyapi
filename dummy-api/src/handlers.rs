pub mod auth {
    use crate::auth::{generate_token, DB};
    use crate::models::profile::{Credentials, Profile, PROFILES};
    use crate::models::Response;
    use serde_json::json;
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn login(credentials: Credentials) -> Result<impl warp::Reply, Infallible> {
        log::debug!("auth_login: {:?}", credentials);

        let db = DB.get().expect("No configured DB.");

        let db = db.lock().await;
        let docs = db.get(PROFILES).unwrap();

        for doc in docs.iter() {
            let account: Profile = bincode::deserialize(&doc).unwrap();
            if account.username == credentials.username && account.password == credentials.password
            {
                let json = warp::reply::json(&Response {
                    data: json!({
                        "id": account.id,
                        "token": generate_token(account.id).unwrap(),
                        "role": account.kind,
                    }),
                    error: json!(null),
                });
                return Ok(warp::reply::with_status(json, StatusCode::OK));
            }
        }

        let json = warp::reply::json(&Response {
            data: json!(null),
            error: json!("Invalid username or password!"),
        });
        Ok(warp::reply::with_status(json, StatusCode::UNAUTHORIZED))
    }
}

pub mod profile {
    use crate::models::profile::{Profile, PROFILES};
    use crate::models::Response;
    use crate::store::Db;
    use serde_json::json;
    use std::convert::Infallible;
    use std::convert::TryFrom;
    use warp::http::StatusCode;

    pub async fn create(mut profile: Profile, db: Db) -> Result<impl warp::Reply, Infallible> {
        log::debug!("profile_create: {:?}", profile);

        let mut db = db.lock().await;

        let docs: &mut Vec<Vec<u8>> = db.get_mut(PROFILES).unwrap();

        match u8::try_from(docs.len()) {
            Ok(v) => profile.id = v + 1,
            Err(_) => {
                let json = warp::reply::json(&Response {
                    data: json!(null),
                    error: json!("Unable to provide profile ID."),
                });
                return Ok(warp::reply::with_status(
                    json,
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        }

        for doc in docs.iter() {
            let account: Profile = bincode::deserialize(&doc).unwrap();
            if account.username == profile.username {
                let json = warp::reply::json(&Response {
                    data: json!(null),
                    error: json!("Username is no longer available!"),
                });
                return Ok(warp::reply::with_status(json, StatusCode::BAD_REQUEST));
            }
        }

        let id = profile.id;
        let kind = profile.kind.clone();

        let data: Vec<u8> = bincode::serialize(&profile).unwrap();
        docs.push(data);

        let json = warp::reply::json(&Response {
            data: json!({ "id": id, "type": kind }),
            error: json!(null),
        });
        Ok(warp::reply::with_status(json, StatusCode::CREATED))
    }
}

pub mod course {
    use crate::auth;
    use crate::models::course::{Course, COURSES};
    use crate::models::profile;
    use crate::models::Response;
    use crate::store::Db;
    use serde_json::json;
    use std::convert::Infallible;
    use std::convert::TryFrom;
    use warp::http::StatusCode;

    pub async fn create(
        course: Course,
        db: Db,
        user: auth::User,
    ) -> Result<impl warp::Reply, Infallible> {
        log::debug!("course_create: {:?}", course);

        if user.id == 0 {
            let json = warp::reply::json(&Response {
                data: json!(null),
                error: json!("Not authorized!"),
            });
            return Ok(warp::reply::with_status(json, StatusCode::UNAUTHORIZED));
        }

        match user.role {
            profile::Kind::Trainee | profile::Kind::Mentor => {
                let json = warp::reply::json(&Response {
                    data: json!(null),
                    error: json!("Forbidden!"),
                });
                return Ok(warp::reply::with_status(json, StatusCode::FORBIDDEN));
            }
            _ => {}
        }

        let mut course = course.with_creator_id(user.id);

        let mut db = db.lock().await;

        let docs: &mut Vec<Vec<u8>> = db.get_mut(COURSES).unwrap();

        match u8::try_from(docs.len()) {
            Ok(v) => course.id = v + 1,
            Err(_) => {
                let json = warp::reply::json(&Response {
                    data: json!(null),
                    error: json!("Unable to provide course ID."),
                });
                return Ok(warp::reply::with_status(
                    json,
                    StatusCode::INTERNAL_SERVER_ERROR,
                ));
            }
        }

        for doc in docs.iter() {
            let existing: Course = bincode::deserialize(&doc).unwrap();
            if existing.title == course.title {
                let json = warp::reply::json(&Response {
                    data: json!(null),
                    error: json!("Title is no longer available!"),
                });
                return Ok(warp::reply::with_status(json, StatusCode::BAD_REQUEST));
            }
        }

        docs.push(bincode::serialize(&course).unwrap());

        let json = warp::reply::json(&Response {
            data: json!(course),
            error: json!(null),
        });
        Ok(warp::reply::with_status(json, StatusCode::CREATED))
    }

    pub async fn update(
        id: u8,
        course: Course,
        db: Db,
        user: auth::User,
    ) -> Result<impl warp::Reply, Infallible> {
        log::debug!("course_create: {:?}", course);

        if user.id == 0 {
            let json = warp::reply::json(&Response {
                data: json!(null),
                error: json!("Not authorized!"),
            });
            return Ok(warp::reply::with_status(json, StatusCode::UNAUTHORIZED));
        }

        match user.role {
            profile::Kind::Trainee | profile::Kind::Mentor => {
                let json = warp::reply::json(&Response {
                    data: json!(null),
                    error: json!("Forbidden!"),
                });
                return Ok(warp::reply::with_status(json, StatusCode::FORBIDDEN));
            }
            _ => {}
        }

        let mut db = db.lock().await;

        let docs: &mut Vec<Vec<u8>> = db.get_mut(COURSES).unwrap();

        for doc in docs.iter_mut() {
            let existing: Course = bincode::deserialize(&doc).unwrap();
            if existing.id == id {
                let creator_id = existing.creator_id;

                let mut existing = course.clone();

                existing.id = id;
                existing.creator_id = creator_id;

                *doc = bincode::serialize(&existing).unwrap();

                let json = warp::reply::json(&Response {
                    data: json!(existing),
                    error: json!(null),
                });
                return Ok(warp::reply::with_status(json, StatusCode::OK));
            }
        }

        let json = warp::reply::json(&Response {
            data: json!(null),
            error: json!("Course not found!"),
        });
        Ok(warp::reply::with_status(json, StatusCode::NOT_FOUND))
    }
}
