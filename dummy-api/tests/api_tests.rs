use warp::http::StatusCode;
use warp::test::request;

use dummy_api::{
    auth, config, course as course_filter,
    models::course::{self, Course},
    models::profile::{self, Credentials, Kind, Profile},
    profile as profile_filter,
};

#[tokio::test]
async fn test_login() {
    config::CONFIG
        .set(config::Config {
            jwt_secret: "secret_key".as_bytes(),
        })
        .expect("Error setting application configuration.");

    let db = profile::new_db();
    let profile = Profile::new()
        .with_id(123)
        .with_username(String::from("mara"))
        .with_password(String::from("secret"));

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
    let db = course::new_db();

    let api = course_filter::courses(db);

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

    assert_eq!(resp.status(), StatusCode::CREATED);
    assert_eq!(resp.body(), "{\"data\":{\"creator_id\":1,\"description\":\"The most recommended training for Rust developers.\",\"id\":1,\"title\":\"Rust in Action\"}}");

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

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(resp.body(), "{\"error\":\"Title is no longer available!\"}");
}
