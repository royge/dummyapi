pub mod auth {
    use crate::models::profile::{Credentials, Db};
    use crate::models::Response;
    use serde_json::json;
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn login(credentials: Credentials, db: Db) -> Result<impl warp::Reply, Infallible> {
        log::debug!("auth_login: {:?}", credentials);

        let vec = db.lock().await;

        for account in vec.iter() {
            if account.username == credentials.username && account.password == credentials.password
            {
                let json = warp::reply::json(&Response {
                    data: json!({ "id": account.id }),
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
    use crate::models::profile::{Db, Profile};
    use crate::models::Response;
    use serde_json::json;
    use std::convert::Infallible;
    use std::convert::TryFrom;
    use warp::http::StatusCode;

    pub async fn create(mut profile: Profile, db: Db) -> Result<impl warp::Reply, Infallible> {
        log::debug!("profile_create: {:?}", profile);

        let mut vec = db.lock().await;

        match u8::try_from(vec.len()) {
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

        for account in vec.iter() {
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

        vec.push(profile);

        let json = warp::reply::json(&Response {
            data: json!({ "id": id, "type": kind }),
            error: json!(null),
        });
        Ok(warp::reply::with_status(json, StatusCode::CREATED))
    }
}
