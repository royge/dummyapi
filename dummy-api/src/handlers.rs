pub mod apiresponse {
    use crate::models::Response;
    use serde_json::json;
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub fn unauthorized() -> Result<warp::reply::WithStatus<warp::reply::Json>, Infallible> {
        let json = warp::reply::json(&Response {
            data: json!(null),
            error: json!("Not authorized!"),
        });
        return Ok(warp::reply::with_status(json, StatusCode::UNAUTHORIZED));
    }

    pub fn forbidden() -> Result<warp::reply::WithStatus<warp::reply::Json>, Infallible> {
        let json = warp::reply::json(&Response {
            data: json!(null),
            error: json!("Forbidden!"),
        });
        return Ok(warp::reply::with_status(json, StatusCode::FORBIDDEN));
    }

    pub fn internal_server_error(
        message: &str,
    ) -> Result<warp::reply::WithStatus<warp::reply::Json>, Infallible> {
        let json = warp::reply::json(&Response {
            data: json!(null),
            error: json!(message),
        });
        return Ok(warp::reply::with_status(
            json,
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    pub fn bad_request(
        message: &str,
    ) -> Result<warp::reply::WithStatus<warp::reply::Json>, Infallible> {
        let json = warp::reply::json(&Response {
            data: json!(null),
            error: json!(message),
        });
        return Ok(warp::reply::with_status(json, StatusCode::BAD_REQUEST));
    }

    pub fn not_found(
        message: &str,
    ) -> Result<warp::reply::WithStatus<warp::reply::Json>, Infallible> {
        let json = warp::reply::json(&Response {
            data: json!(null),
            error: json!(message),
        });
        return Ok(warp::reply::with_status(json, StatusCode::NOT_FOUND));
    }
}

pub mod auth {
    use crate::auth::generate_token;
    use crate::models::profile::{Credentials, Profile, PROFILES};
    use crate::models::Response;
    use crate::store::Db;
    use serde_json::json;
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn login(credentials: Credentials, db: Db) -> Result<impl warp::Reply, Infallible> {
        log::debug!("auth_login: {:?}", credentials);

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
    use crate::handlers::apiresponse;
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
                return apiresponse::internal_server_error("Unable to provide profile ID.");
            }
        }

        for doc in docs.iter() {
            let account: Profile = bincode::deserialize(&doc).unwrap();
            if account.username == profile.username {
                return apiresponse::bad_request("Username is no longer available!");
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
    use crate::handlers::apiresponse;
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
            return apiresponse::unauthorized();
        }

        match user.role {
            profile::Kind::Trainee | profile::Kind::Mentor => {
                return apiresponse::forbidden();
            }
            _ => {}
        }

        let mut course = course.with_creator_id(user.id);

        let mut db = db.lock().await;

        let docs: &mut Vec<Vec<u8>> = db.get_mut(COURSES).unwrap();

        match u8::try_from(docs.len()) {
            Ok(v) => course.id = v + 1,
            Err(_) => {
                return apiresponse::internal_server_error("Unable to provide course ID.");
            }
        }

        for doc in docs.iter() {
            let existing: Course = bincode::deserialize(&doc).unwrap();
            if existing.title == course.title {
                return apiresponse::bad_request("Title is no longer available!");
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
        log::debug!("course_update: {:?}", course);

        if user.id == 0 {
            return apiresponse::unauthorized();
        }

        match user.role {
            profile::Kind::Trainee | profile::Kind::Mentor => {
                return apiresponse::forbidden();
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

        apiresponse::not_found("Course not found!")
    }
}

pub mod topic {
    use crate::handlers::apiresponse;
    use crate::models::profile;
    use crate::models::topic::{Topic, TOPICS};
    use crate::models::Response;
    use crate::store::Db;
    use crate::{auth, course};
    use serde_json::json;
    use std::convert::Infallible;
    use std::convert::TryFrom;
    use warp::http::StatusCode;

    pub async fn create(
        topic: Topic,
        db: Db,
        user: auth::User,
    ) -> Result<impl warp::Reply, Infallible> {
        log::debug!("topic_create: {:?}", topic);

        if user.id == 0 {
            return apiresponse::unauthorized();
        }

        match user.role {
            profile::Kind::Trainee => {
                return apiresponse::forbidden();
            }
            _ => {}
        }

        if course::get(topic.course_id, &db).await.is_err() {
            return apiresponse::bad_request("Course not found!");
        }

        let mut topic = topic.with_creator_id(user.id);

        let mut db = db.lock().await;

        let docs: &mut Vec<Vec<u8>> = db.get_mut(TOPICS).unwrap();

        match u8::try_from(docs.len()) {
            Ok(v) => topic.id = v + 1,
            Err(_) => {
                return apiresponse::internal_server_error("Unable to provide topics ID.");
            }
        }

        for doc in docs.iter() {
            let existing: Topic = bincode::deserialize(&doc).unwrap();

            let same_course = existing.course_id == topic.course_id;
            let same_title = existing.title == topic.title;

            if same_course && same_title {
                return apiresponse::bad_request("Title is no longer available!");
            }
        }

        docs.push(bincode::serialize(&topic).unwrap());

        let json = warp::reply::json(&Response {
            data: json!(topic),
            error: json!(null),
        });
        Ok(warp::reply::with_status(json, StatusCode::CREATED))
    }

    pub async fn update(
        id: u8,
        topic: Topic,
        db: Db,
        user: auth::User,
    ) -> Result<impl warp::Reply, Infallible> {
        log::debug!("topic_update: {:?}", topic);

        if user.id == 0 {
            return apiresponse::unauthorized();
        }

        match user.role {
            profile::Kind::Trainee => {
                return apiresponse::forbidden();
            }
            _ => {}
        }

        let mut db = db.lock().await;

        let docs: &mut Vec<Vec<u8>> = db.get_mut(TOPICS).unwrap();

        for doc in docs.iter_mut() {
            let existing: Topic = bincode::deserialize(&doc).unwrap();
            if existing.id == id {
                let creator_id = existing.creator_id;
                let course_id = existing.course_id;

                let mut existing = topic.clone();

                existing.id = id;
                existing.creator_id = creator_id;
                existing.course_id = course_id;

                *doc = bincode::serialize(&existing).unwrap();

                let json = warp::reply::json(&Response {
                    data: json!(existing),
                    error: json!(null),
                });
                return Ok(warp::reply::with_status(json, StatusCode::OK));
            }
        }

        apiresponse::not_found("Topic not found!")
    }
}
