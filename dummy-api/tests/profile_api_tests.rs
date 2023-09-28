use warp::http::StatusCode;
use warp::test::request;

use dummy_api::{
    models::profile::{self, Kind, Profile},
    profile as profile_filter, store,
};

#[tokio::test]
async fn test_create_profile() {
    let db = store::new_db(vec![profile::PROFILES]).await;

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

    // create profile with existing username
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
