use super::config::CONFIG;
use super::handlers;
use super::models::profile::{get_kind, Credentials, Kind, Profile};
use super::store::Db;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use rand::Rng;
use serde_derive::{Deserialize, Serialize};
use std::convert::Infallible;
use warp::{Filter, Rejection};

pub fn auth(db: Db) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    login(db.clone())
}

pub fn login(
    db: Db,
) -> impl Filter<Extract = (impl warp::Reply,), Error = warp::Rejection> + Clone {
    warp::path!("auth")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::auth::login)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (Credentials,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    user_id: u8,
    exp: i64,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u8,
    pub role: Kind,
}

impl User {
    pub fn can_view(&self, profile: &Profile) -> bool {
        if self.id == profile.id {
            return true;
        }

        if self.role == Kind::Admin {
            return true;
        }

        if self.role == Kind::Mentor {
            return profile.kind == Kind::Trainee;
        }

        false
    }
}

pub fn generate_token(user_id: u8) -> Result<String, Box<dyn std::error::Error>> {
    let expiration = Utc::now() + Duration::hours(1);
    let claims = Claims {
        user_id,
        exp: expiration.timestamp(),
    };

    let config = CONFIG
        .get()
        .expect("Application is not properly configured.");

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.jwt_secret),
    )?;
    Ok(token)
}

pub fn generate_secret_key(length: usize) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let key: Vec<u8> = (0..length).map(|_| rng.gen()).collect();
    key
}

fn decode_token(token: &str) -> Result<u8, Rejection> {
    let config = CONFIG
        .get()
        .expect("Application is not properly configured.");

    let token = token.replace('\"', "");

    let validation = Validation::new(Algorithm::HS256);
    let token_message = decode::<Claims>(
        &token,
        &DecodingKey::from_secret(config.jwt_secret),
        &validation,
    );

    match token_message {
        Ok(data) => Ok(data.claims.user_id),
        Err(_) => Err(warp::reject()),
    }
}

pub fn with_auth(db: Db) -> impl Filter<Extract = (User,), Error = Rejection> + Clone {
    warp::header::<String>("Authorization")
        .and_then(move |auth_header: String| {
            let db = db.clone();
            async move {
                let token = auth_header.replace("Bearer ", "");

                match decode_token(&token) {
                    Ok(user_id) => {
                        if let Ok(kind) = get_kind(&db, user_id).await {
                            return Ok(User {
                                id: user_id,
                                role: kind,
                            });
                        }
                        Ok(User {
                            id: user_id,
                            role: Kind::Trainee,
                        })
                    }
                    Err(_) => Err(warp::reject()),
                }
            }
        })
        .or_else(|_| async {
            Ok::<_, warp::Rejection>((User {
                id: 0,
                role: Kind::Trainee,
            },))
        })
}

#[tokio::test]
async fn test_jwt_encode_decode() {
    use super::config::{Config, CONFIG};

    CONFIG
        .set(Config {
            jwt_secret: "secret_key".as_bytes(),
        })
        .expect("Error setting application configuration.");

    let token = generate_token(123).unwrap();
    assert_ne!(token, "");

    let user_id = decode_token(&token).unwrap();
    assert_eq!(user_id, 123);
}

#[test]
fn test_user_can_view() {
    let trainee = User {
        id: 1,
        role: Kind::Trainee,
    };

    let admin = User {
        id: 2,
        role: Kind::Admin,
    };

    let mentor = User {
        id: 3,
        role: Kind::Mentor,
    };

    let trainee_profile = Profile { id: trainee.id, ..Default::default() };
    let mentor_profile = Profile { id: mentor.id, kind: Kind::Mentor, ..Default::default() };
    let admin_profile = Profile { id: admin.id, kind: Kind::Admin, ..Default::default() };

    assert!(trainee.can_view(&trainee_profile));
    assert!(admin.can_view(&trainee_profile));
    assert!(mentor.can_view(&trainee_profile));

    assert!(!trainee.can_view(&mentor_profile));
    assert!(!trainee.can_view(&admin_profile));
    assert!(admin.can_view(&mentor_profile));
    assert!(mentor.can_view(&mentor_profile));
    assert!(!mentor.can_view(&admin_profile));
}
