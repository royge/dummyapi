use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=auth=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "todos=info");
    }

    pretty_env_logger::init();

    let db = models::new_db();

    let profiles = [
        models::Profile {
            id: 1,
            username: String::from("tim"),
            password: String::from("secret"),
        },
        models::Profile {
            id: 2,
            username: String::from("mara"),
            password: String::from("secret"),
        },
        models::Profile {
            id: 3,
            username: String::from("deno"),
            password: String::from("secret"),
        },
        models::Profile {
            id: 4,
            username: String::from("ferris"),
            password: String::from("secret"),
        },
        models::Profile {
            id: 5,
            username: String::from("david"),
            password: String::from("secret"),
        },
    ];

    models::initialize(db.clone(), &profiles).await;

    let api = auth::auth(db);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type", "Authorization"])
        .allow_methods(vec!["POST"]);

    // View access logs by setting `RUST_LOG=auth`.
    let routes = api.with(cors).with(warp::log("auth"));

    let host = [127, 0, 0, 1];
    let port = 3030;

    let host_string = host
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>()
        .join(".");
    println!("Starting dummy server at http://{}:{}", host_string, &port);

    show_credentials(&profiles);

    // Start up the server...
    warp::serve(routes).run((host, port)).await;
}

fn show_credentials(profiles: &[models::Profile]) {
    println!("\nYou can login using any of the following credentials.\n");
    for p in profiles {
        println!("\tusername: {}\n\tpassword: {}\n", p.username, p.password);
    }
}

mod auth {
    use super::handlers;
    use super::models::{Credentials, Db};
    use std::convert::Infallible;
    use warp::Filter;

    pub fn auth(
        db: Db,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        login(db)
    }

    pub fn login(
        db: Db,
    ) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
        warp::path!("auth")
            .and(warp::post())
            .and(json_body())
            .and(with_db(db))
            .and_then(handlers::login)
    }

    fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
        warp::any().map(move || db.clone())
    }

    fn json_body() -> impl Filter<Extract = (Credentials,), Error = warp::Rejection> + Clone {
        // When accepting a body, we want a JSON body
        // (and to reject huge payloads)...
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

mod handlers {
    use super::models::{Credentials, Db, Response};
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

mod models {
    use serde_derive::{Deserialize, Serialize};
    use serde_json::Value;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    pub type Db = Arc<Mutex<Vec<Profile>>>;

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Profile {
        pub id: u8,
        pub username: String,
        pub password: String,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Credentials {
        pub username: String,
        pub password: String,
    }

    pub fn new_db() -> Db {
        Arc::new(Mutex::new(Vec::new()))
    }

    pub async fn initialize(db: Db, list: &[Profile]) {
        let mut vec = db.lock().await;

        for profs in list {
            vec.push(profs.clone());
        }
    }

    #[derive(Serialize)]
    pub struct Response {
        #[serde(skip_serializing_if = "Value::is_null")]
        pub data: Value,

        #[serde(skip_serializing_if = "Value::is_null")]
        pub error: Value,
    }
}

#[cfg(test)]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;

    use super::{
        auth,
        models::{self, Credentials},
    };

    #[tokio::test]
    async fn test_login() {
        let db = models::new_db();
        let profile = models::Profile {
            id: 123,
            username: String::from("mara"),
            password: String::from("secret"),
        };

        models::initialize(db.clone(), &[profile]).await;

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
        assert_eq!(resp.body(), "{\"data\":{\"id\":123}}");

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
        assert_eq!(
            resp.body(),
            "{\"error\":\"Invalid username or password!\"}"
        );
    }
}
