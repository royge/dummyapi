use warp::http::StatusCode;
use warp::test::request;

use dummy_api::{
    auth, config,
    models::profile::{self, Credentials, Kind, Profile},
    store,
};

#[tokio::test]
async fn test_login() {
    let _ = config::CONFIG.set(config::Config {
        jwt_secret: "secret_key".as_bytes(),
    });

    let db = store::new_db(vec![profile::PROFILES]).await;

    let username = String::from("mara");
    let password = String::from("secret");

    let admin = Profile::new()
        .with_id(123)
        .with_username(username.clone())
        .with_password(password.clone())
        .with_kind(Kind::Admin);

    profile::initialize(&db, &[admin]).await;

    let api = auth::auth(db);

    // login with existing user
    let resp = request()
        .method("POST")
        .path("/auth")
        .json(&Credentials {
            username: username,
            password: password,
        })
        .reply(&api)
        .await;

    assert_eq!(resp.status(), StatusCode::OK);

    // login with non-existing user
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
