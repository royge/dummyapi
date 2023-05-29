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

    let credentials = [
        models::Credentials{
            username: String::from("tim"),
            password: String::from("secret"),
        },
        models::Credentials{
            username: String::from("mara"),
            password: String::from("secret"),
        },
        models::Credentials{
            username: String::from("deno"),
            password: String::from("secret"),
        },
        models::Credentials{
            username: String::from("ferris"),
            password: String::from("secret"),
        },
        models::Credentials{
            username: String::from("david"),
            password: String::from("secret"),
        },
    ];

    models::initialize(db.clone(), &credentials).await;

    let api = auth::auth(db);

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type", "Authorization"])
        .allow_methods(vec!["POST"]);

    // View access logs by setting `RUST_LOG=auth`.
    let routes = api
        .with(cors)
        .with(warp::log("auth"));

    let host = [127, 0, 0, 1];
    let port = 3030;

    let host_string = host.iter().map(|item| item.to_string()).collect::<Vec<String>>().join(".");
    println!("Starting dummy server at http://{}:{}", host_string, &port);

    show_credentials(&credentials);

    // Start up the server...
    warp::serve(routes).run((host, port)).await;
}

fn show_credentials(creds: &[models::Credentials]) {
    println!("\nYou can login using any of the following credentials.\n");
    for c in creds {
        println!("\tusername: {}\n\tpassword: {}\n", c.username, c.password);
    }
}

mod auth {
    use super::handlers;
    use super::models::{Credentials, Db};
    use std::convert::Infallible;
    use warp::Filter;

    pub fn auth(db: Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
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
    use super::models::{Credentials, Db};
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn login(credentials: Credentials, db: Db) -> Result<impl warp::Reply, Infallible> {
        log::debug!("auth_login: {:?}", credentials);

        let vec = db.lock().await;

        for account in vec.iter() {
            if account.username == credentials.username && account.password == credentials.password
            {
                return Ok(StatusCode::OK);
            }
        }

        Ok(StatusCode::UNAUTHORIZED)
    }
}

mod models {
    use serde_derive::{Deserialize, Serialize};
    use std::sync::Arc;
    use tokio::sync::Mutex;

    pub type Db = Arc<Mutex<Vec<Credentials>>>;

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct Credentials {
        pub username: String,
        pub password: String,
    }

    pub fn new_db() -> Db {
        Arc::new(Mutex::new(Vec::new()))
    }

    pub async fn initialize(db: Db, list: &[Credentials]) {
        let mut vec = db.lock().await;

        for creds in list {
            vec.push(creds.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;

    use super::{auth, models::{self, Credentials}};

    #[tokio::test]
    async fn test_login() {
        let db = models::new_db();
        let creds = models::Credentials{
                username: String::from("mara"),
                password: String::from("secret"),
            };

        models::initialize(db.clone(), &[creds]).await;

        let api = auth::auth(db);

        let resp = request()
            .method("POST")
            .path("/auth")
            .json(&Credentials{
                username: String::from("mara"),
                password: String::from("secret"),
            })
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::OK);

        let resp = request()
            .method("POST")
            .path("/auth")
            .json(&Credentials{
                username: String::from("klabnik"),
                password: String::from("secret"),
            })
            .reply(&api)
            .await;

        assert_eq!(resp.status(), StatusCode::UNAUTHORIZED);
    }
}
