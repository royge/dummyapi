use dummy_api::{auth, config, course, models, profile, store, topic};
use hex;
use lazy_static::lazy_static;
use std::env;
use warp::Filter;

#[tokio::main]
async fn main() {
    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=auth=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("RUST_LOG", "todos=info");
    }

    // Generate a random secret key to sign JWT tokens with.
    lazy_static! {
        static ref SECRET_KEY: String = {
            let key = auth::generate_secret_key(32);
            hex::encode(key)
        };
    }

    config::CONFIG
        .set(config::Config {
            jwt_secret: SECRET_KEY.as_bytes(),
        })
        .expect("Error setting application configuration.");

    pretty_env_logger::init();

    let collections = vec![
        models::profile::PROFILES,
        models::course::COURSES,
        models::topic::TOPICS,
    ];

    let db = store::new_db(collections).await;

    let roots = [models::profile::Profile::new()
        .with_id(1)
        .with_username(String::from("root"))
        .with_generated_password()
        .with_kind(models::profile::Kind::Root)];

    models::profile::initialize(&db, &roots).await;

    let api = auth::auth(db.clone())
        .or(profile::profiles(db.clone()))
        .or(course::courses(db.clone()))
        .or(topic::topics(db.clone()));

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Content-Type", "Authorization"])
        .allow_methods(vec!["POST"]);

    // View access logs by setting `RUST_LOG=auth`.
    let routes = api.with(cors).with(warp::log("auth"));

    let host = [0, 0, 0, 0];
    let port = 3030;

    let host_string = host
        .iter()
        .map(|item| item.to_string())
        .collect::<Vec<String>>()
        .join(".");

    println!("Starting dummy server at http://{}:{}", host_string, &port);

    show_root_credentials(&roots);

    // Start up the server...
    warp::serve(routes).run((host, port)).await;
}

fn show_root_credentials(roots: &[models::profile::Profile]) {
    println!("\nYou can login using the following root credentials.\n");
    for p in roots {
        println!("\tusername: {}\n\tpassword: {}\n", p.username, p.password);
    }
}
