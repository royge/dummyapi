use serde_json::Value;
use std::str::from_utf8;
use warp::http::StatusCode;
use warp::test::request;
use warp::Filter;

use dummy_api::{
    auth, config, course as course_filter,
    models::course::{self, Course},
    models::profile::{self, Credentials, Kind, Profile},
    models::topic::{self, Topic},
    store, topic as topic_filter,
};

#[tokio::test]
async fn test_create_topic() {
    let _ = config::CONFIG.set(config::Config {
        jwt_secret: "secret_key".as_bytes(),
    });

    let db = store::new_db(vec![profile::PROFILES, course::COURSES, topic::TOPICS]).await;

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

    let mentor = Profile::new()
        .with_id(125)
        .with_username(String::from("nara"))
        .with_password(String::from("secret"))
        .with_kind(Kind::Mentor);

    profile::initialize(&db, &[admin, trainee, mentor]).await;

    let api = auth::auth(db.clone())
        .or(course_filter::courses(db.clone()))
        .or(topic_filter::topics(db));

    let resp = request()
        .method("POST")
        .path("/topics")
        .json(
            &Topic::new()
                .with_title(String::from("Rust in Action"))
                .with_description(String::from(
                    "The most recommended training for Rust developers.",
                ))
                .with_course_id(1),
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

    // admin create course with unknown course
    let resp = request()
        .method("POST")
        .header("Authorization", authorization.clone())
        .path("/topics")
        .json(
            &Topic::new()
                .with_title(String::from("Rust in Action"))
                .with_description(String::from(
                    "The most recommended training for Rust developers.",
                ))
                .with_course_id(1),
        )
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    assert_eq!(resp.body(), "{\"error\":\"Course not found!\"}");

    let authorization = authorization.clone();

    // admin create course
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

    // parse course id
    let data = from_utf8(resp.body()).unwrap();
    let value: Value = serde_json::from_str(data).unwrap();
    let course_id = value["data"]["id"].as_i64().unwrap();

    // admin create course with existing course
    let resp = request()
        .method("POST")
        .header("Authorization", authorization.clone())
        .path("/topics")
        .json(
            &Topic::new()
                .with_title(String::from("Rust in Action"))
                .with_description(String::from(
                    "The most recommended training for Rust developers.",
                ))
                .with_course_id(course_id.try_into().unwrap()),
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
        .path("/topics")
        .json(
            &Topic::new()
                .with_title(String::from("Rust in Actions"))
                .with_description(String::from(
                    "The most recommended training for Rust developers.",
                ))
                .with_course_id(1),
        )
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::FORBIDDEN);

    // mentor login
    let resp = request()
        .method("POST")
        .path("/auth")
        .json(&Credentials {
            username: String::from("nara"),
            password: String::from("secret"),
        })
        .reply(&api)
        .await;

    let data = from_utf8(resp.body()).unwrap();
    let value: Value = serde_json::from_str(data).unwrap();
    let authorization = format!("Bearer {}", value["data"]["token"]);

    // mentor create topic with existing course
    let resp = request()
        .method("POST")
        .header("Authorization", authorization)
        .path("/topics")
        .json(
            &Topic::new()
                .with_title(String::from("Rust in Action - 2nd Edition"))
                .with_description(String::from(
                    "The most recommended training for Rust developers.",
                ))
                .with_course_id(course_id.try_into().unwrap()),
        )
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::CREATED);
}

#[tokio::test]
async fn test_update_topic() {
    let _ = config::CONFIG.set(config::Config {
        jwt_secret: "secret_key".as_bytes(),
    });

    let db = store::new_db(vec![profile::PROFILES, course::COURSES, topic::TOPICS]).await;

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

    let mentor = Profile::new()
        .with_id(125)
        .with_username(String::from("nara"))
        .with_password(String::from("secret"))
        .with_kind(Kind::Mentor);

    profile::initialize(&db, &[admin, trainee, mentor]).await;

    let api = auth::auth(db.clone())
        .or(course_filter::courses(db.clone()))
        .or(topic_filter::topics(db));

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

    let course_id = value["data"]["id"].as_i64().unwrap();

    // admin create topic
    let resp = request()
        .method("POST")
        .header("Authorization", authorization.clone())
        .path("/topics")
        .json(
            &Topic::new()
                .with_title(String::from("Rust in Action"))
                .with_description(String::from(
                    "The most recommended training for Rust developers.",
                ))
                .with_course_id(course_id.try_into().unwrap()),
        )
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::CREATED);
    let data = from_utf8(resp.body()).unwrap();
    let value: Value = serde_json::from_str(data).unwrap();

    let topic_id = value["data"]["id"].as_i64().unwrap();

    // admin update topic
    let _ = request()
        .method("PUT")
        .header("Authorization", authorization.clone())
        .path(&format!("/topics/{}", topic_id))
        .json(
            &Topic::new()
                .with_title(String::from("Rust in Production"))
                .with_description(String::from(
                    "The most recommended training for Rust developers.",
                ))
                .with_course_id(course_id.try_into().unwrap()),
        )
        .reply(&api)
        .await;

    // mentor login
    let resp = request()
        .method("POST")
        .path("/auth")
        .json(&Credentials {
            username: String::from("nara"),
            password: String::from("secret"),
        })
        .reply(&api)
        .await;

    let data = from_utf8(resp.body()).unwrap();
    let value: Value = serde_json::from_str(data).unwrap();
    let authorization = format!("Bearer {}", value["data"]["token"]);

    // mentor update topic
    let resp = request()
        .method("PUT")
        .header("Authorization", authorization)
        .path(&format!("/topics/{}", topic_id))
        .json(
            &Topic::new()
                .with_title(String::from("Rust in Production"))
                .with_description(String::from(
                    "The most recommended training for Rust developers.",
                )),
        )
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);

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

    // trainee update topic
    let resp = request()
        .method("PUT")
        .header("Authorization", authorization)
        .path(&format!("/topics/{}", topic_id))
        .json(
            &Topic::new()
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
async fn test_list_topics() {
    let _ = config::CONFIG.set(config::Config {
        jwt_secret: "secret_key".as_bytes(),
    });

    let db = store::new_db(vec![profile::PROFILES, course::COURSES, topic::TOPICS]).await;

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

    let mentor = Profile::new()
        .with_id(125)
        .with_username(String::from("nara"))
        .with_password(String::from("secret"))
        .with_kind(Kind::Mentor);

    profile::initialize(&db, &[admin, trainee, mentor]).await;

    let api = auth::auth(db.clone())
        .or(course_filter::courses(db.clone()))
        .or(topic_filter::topics(db));

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

    let course_id = value["data"]["id"].as_i64().unwrap();

    let total = 20;

    // admin create topics
    for i in 1..total + 1 {
        let resp = request()
            .method("POST")
            .header("Authorization", authorization.clone())
            .path("/topics")
            .json(
                &Topic::new()
                    .with_title(format!("Borrowing - 1{}th Chapter", i))
                    .with_description(String::from(
                        "The most recommended training for Rust developers.",
                    ))
                    .with_course_id(course_id.try_into().unwrap()),
            )
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::CREATED);
    }

    // admin get topics
    let resp = request()
        .method("GET")
        .header("Authorization", authorization.clone())
        .path(&format!("/topics?course_id={}", course_id))
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);

    let data = from_utf8(resp.body()).unwrap();
    let value: Value = serde_json::from_str(data).unwrap();

    assert_eq!(value["data"].as_array().unwrap().len(), total);

    // admin get topics with unknown course id
    let uknown_id = 99;
    let resp = request()
        .method("GET")
        .header("Authorization", authorization.clone())
        .path(&format!("/topics?course_id={}", uknown_id))
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);

    let data = from_utf8(resp.body()).unwrap();
    let value: Value = serde_json::from_str(data).unwrap();

    assert_eq!(value["data"].as_array().unwrap().len(), 0);
}
