use serde_json::Value;
use std::str::from_utf8;
use warp::http::StatusCode;
use warp::test::request;
use warp::Filter;

use dummy_api::{
    auth, config, course as course_filter,
    models::course::{self, Course},
    models::profile::{self, Credentials, Kind, Profile},
    profile as profile_filter,
};

#[tokio::test]
async fn test_login() {
    let _ = config::CONFIG.set(config::Config {
        jwt_secret: "secret_key".as_bytes(),
    });

    let db = profile::new_db();
    let profile = Profile::new()
        .with_id(123)
        .with_username(String::from("mara"))
        .with_password(String::from("secret"))
        .with_kind(Kind::Admin);

    profile::initialize(db.clone(), &[profile]).await;

    let api = auth::auth(db);

    let resp = request()
        .method("POST")
        .path("/auth")
        .json(&Credentials {
            username: String::from("mara"),
            password: String::from("secret"),
        })
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);
    // assert_eq!(resp.body(), "{\"data\":{\"id\":123}}");

    let resp = request()
        .method("POST")
        .path("/auth")
        .json(&Credentials {
            username: String::from("klabnik"),
            password: String::from("secret"),
        })
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(resp.body(), "{\"error\":\"Invalid username or password!\"}");
}

#[tokio::test]
async fn test_create_profile() {
    let db = profile::new_db();

    let api = profile_filter::profiles(db);

    let resp = request()
        .method("POST")
        .path("/profiles")
        .json(
            &Profile::new()
                .with_username(String::from("deitel"))
                .with_password(String::from("secret"))
                .with_first_name(String::from("Paul"))
                .with_last_name(String::from("Deitel"))
                .with_kind(Kind::Admin),
        )
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::CREATED);
    assert_eq!(resp.body(), "{\"data\":{\"id\":1,\"type\":\"admin\"}}");

    let resp = request()
        .method("POST")
        .path("/profiles")
        .json(
            &Profile::new()
                .with_username(String::from("deitel"))
                .with_password(String::from("secret")),
        )
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        resp.body(),
        "{\"error\":\"Username is no longer available!\"}"
    );
}

#[tokio::test]
async fn test_create_course() {
    let _ = config::CONFIG.set(config::Config {
        jwt_secret: "secret_key".as_bytes(),
    });

    let db = course::new_db();
    let profile_db = profile::new_db();

    let admin = Profile::new()
        .with_id(123)
        .with_username(String::from("mara"))
        .with_password(String::from("secret"))
        .with_kind(Kind::Admin);

    let trainee = Profile::new()
        .with_id(124)
        .with_username(String::from("dara"))
        .with_password(String::from("secret"))
        .with_kind(Kind::Trainee);

    profile::initialize(profile_db.clone(), &[admin, trainee]).await;

    let api = auth::auth(profile_db.clone()).or(course_filter::courses(db));

    let resp = request()
        .method("POST")
        .path("/courses")
        .json(
            &Course::new()
                .with_title(String::from("Rust in Action"))
                .with_description(String::from(
                    "The most recommended training for Rust developers.",
                ))
                .with_creator_id(1),
        )
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    assert_eq!(resp.body(), "{\"error\":\"Not authorized!\"}");

    // admin login
    let resp = request()
        .method("POST")
        .path("/auth")
        .json(&Credentials {
            username: String::from("mara"),
            password: String::from("secret"),
        })
        .reply(&api)
        .await;

    let data = from_utf8(resp.body()).unwrap();
    let value: Value = serde_json::from_str(data).unwrap();
    let authorization = format!("Bearer {}", value["data"]["token"]);

    let resp = request()
        .method("POST")
        .header("Authorization", authorization)
        .path("/courses")
        .json(
            &Course::new()
                .with_title(String::from("Rust in Action"))
                .with_description(String::from(
                    "The most recommended training for Rust developers.",
                ))
                .with_creator_id(1),
        )
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::CREATED);

    // trainee login
    let resp = request()
        .method("POST")
        .path("/auth")
        .json(&Credentials {
            username: String::from("dara"),
            password: String::from("secret"),
        })
        .reply(&api)
        .await;

    let data = from_utf8(resp.body()).unwrap();
    let value: Value = serde_json::from_str(data).unwrap();
    let authorization = format!("Bearer {}", value["data"]["token"]);

    let resp = request()
        .method("POST")
        .header("Authorization", authorization)
        .path("/courses")
        .json(
            &Course::new()
                .with_title(String::from("Rust in Actions"))
                .with_description(String::from(
                    "The most recommended training for Rust developers.",
                ))
                .with_creator_id(1),
        )
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_update_course() {
    let _ = config::CONFIG.set(config::Config {
        jwt_secret: "secret_key".as_bytes(),
    });

    let db = course::new_db();
    let profile_db = profile::new_db();

    let admin = Profile::new()
        .with_id(123)
        .with_username(String::from("mara"))
        .with_password(String::from("secret"))
        .with_kind(Kind::Admin);

    let trainee = Profile::new()
        .with_id(124)
        .with_username(String::from("dara"))
        .with_password(String::from("secret"))
        .with_kind(Kind::Trainee);

    profile::initialize(profile_db.clone(), &[admin, trainee]).await;

    let api = auth::auth(profile_db.clone()).or(course_filter::courses(db));

    // admin login
    let resp = request()
        .method("POST")
        .path("/auth")
        .json(&Credentials {
            username: String::from("mara"),
            password: String::from("secret"),
        })
        .reply(&api)
        .await;

    let data = from_utf8(resp.body()).unwrap();
    let value: Value = serde_json::from_str(data).unwrap();
    let authorization = format!("Bearer {}", value["data"]["token"]);

    let resp = request()
        .method("POST")
        .header("Authorization", authorization.clone())
        .path("/courses")
        .json(
            &Course::new()
                .with_title(String::from("Rust in Action"))
                .with_description(String::from(
                    "The most recommended training for Rust developers.",
                ))
                .with_creator_id(1),
        )
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::CREATED);

    let data = from_utf8(resp.body()).unwrap();
    let value: Value = serde_json::from_str(data).unwrap();

    let _ = request()
        .method("PUT")
        .header("Authorization", authorization)
        .path(&format!("/courses/{}", value["data"]["id"]))
        .json(
            &Course::new()
                .with_title(String::from("Rust in Production"))
                .with_description(String::from(
                    "The most recommended training for Rust developers.",
                ))
                .with_creator_id(1),
        )
        .reply(&api)
        .await;

    // trainee login
    let resp = request()
        .method("POST")
        .path("/auth")
        .json(&Credentials {
            username: String::from("dara"),
            password: String::from("secret"),
        })
        .reply(&api)
        .await;

    let data = from_utf8(resp.body()).unwrap();
    let value: Value = serde_json::from_str(data).unwrap();
    let authorization = format!("Bearer {}", value["data"]["token"]);

    let resp = request()
        .method("PUT")
        .header("Authorization", authorization)
        .path(&format!("/courses/{}", value["data"]["id"]))
        .json(
            &Course::new()
                .with_title(String::from("Rust in Production"))
                .with_description(String::from(
                    "The most recommended training for Rust developers.",
                ))
        )
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}
