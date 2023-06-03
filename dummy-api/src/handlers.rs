use super::models::{Credentials, Db, Response};
use serde_json::json;
use std::convert::Infallible;
use warp::http::StatusCode;

pub async fn login(credentials: Credentials, db: Db) -> Result<impl warp::Reply, Infallible> {
    log::debug!("auth_login: {:?}", credentials);

    let vec = db.lock().await;

    for account in vec.iter() {
        if account.username == credentials.username && account.password == credentials.password {
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
