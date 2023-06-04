use std::env;
use warp::Filter;
use dummy_api::{auth, profile};
use dummy_api::models;

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
        models::Profile::new()
            .with_id(1)
            .with_username(String::from("tim"))
            .with_password(String::from("secret")),
        models::Profile::new()
            .with_id(2)
            .with_username(String::from("mara"))
            .with_password(String::from("secret")),
        models::Profile::new()
            .with_id(3)
            .with_username(String::from("deno"))
            .with_password(String::from("secret")),
        models::Profile::new()
            .with_id(4)
            .with_username(String::from("ferris"))
            .with_password(String::from("secret")),
        models::Profile::new()
            .with_id(5)
            .with_username(String::from("david"))
            .with_password(String::from("secret")),
    ];

    models::initialize(db.clone(), &profiles).await;

    let api = auth::auth(db.clone())
        .or(profile::profiles(db.clone()));

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
