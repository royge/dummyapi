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
