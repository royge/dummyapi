use serde_json::Value;
use std::str::from_utf8;
use warp::http::StatusCode;
use warp::test::request;
use warp::Filter;

use dummy_api::{
    auth, config, course as course_filter,
    models::course::{self, Course},
    models::profile::{self, Credentials, Kind, Profile},
    store,
};

#[tokio::test]
async fn test_create_course() {
    let _ = config::CONFIG.set(config::Config {
        jwt_secret: "secret_key".as_bytes(),
    });

    let db = store::new_db(vec![profile::PROFILES, course::COURSES]).await;

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

    profile::initialize(&db, &[admin, trainee]).await;

    let api = auth::auth(db.clone()).or(course_filter::courses(db));

    // create course without authorization
    let resp = request()
        .method("POST")
        .path("/courses")
        .json(
            &Course::new()
                .with_title(String::from("Rust in Action"))
                .with_description(String::from(
                    "The most recommended training for Rust developers.",
                )),
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

    // admin create course
    let resp = request()
        .method("POST")
        .header("Authorization", authorization)
        .path("/courses")
        .json(
            &Course::new()
                .with_title(String::from("Rust in Action"))
                .with_description(String::from(
                    "The most recommended training for Rust developers.",
                )),
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

    // trainee try to create course
    let resp = request()
        .method("POST")
        .header("Authorization", authorization)
        .path("/courses")
        .json(
            &Course::new()
                .with_title(String::from("Rust in Actions"))
                .with_description(String::from(
                    "The most recommended training for Rust developers.",
                )),
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

    let db = store::new_db(vec![profile::PROFILES, course::COURSES]).await;

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

    profile::initialize(&db, &[admin, trainee]).await;

    let api = auth::auth(db.clone()).or(course_filter::courses(db));

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
                )),
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
                )),
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
                )),
        )
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_list_courses() {
    let _ = config::CONFIG.set(config::Config {
        jwt_secret: "secret_key".as_bytes(),
    });

    let db = store::new_db(vec![profile::PROFILES, course::COURSES]).await;

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

    profile::initialize(&db, &[admin, trainee]).await;

    let api = auth::auth(db.clone()).or(course_filter::courses(db));

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

    let total = 20;
    // admin create courses
    for i in 1..total + 1 {
        let resp = request()
            .method("POST")
            .header("Authorization", authorization.clone())
            .path("/courses")
            .json(
                &Course::new()
                    .with_title(format!("Rust in Action - 1{}th Edition", i))
                    .with_description(String::from(
                        "The most recommended training for Rust developers.",
                    )),
            )
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::CREATED);
    }

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

    // trainee view courses
    let resp = request()
        .method("GET")
        .header("Authorization", authorization)
        .path("/courses")
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);

    let data = from_utf8(resp.body()).unwrap();
    let value: Value = serde_json::from_str(data).unwrap();
    assert_eq!(value["data"].as_array().unwrap().len(), total);
}
