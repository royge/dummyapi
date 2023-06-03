use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type Db = Arc<Mutex<Vec<Profile>>>;

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct Profile {
    pub id: u8,
    pub username: String,
    pub password: String,
    pub first_name: String,
    pub last_name: String,
    pub kind: Kind,
}

impl Profile {
    pub fn new() -> Profile {
        Profile::default()
    }

    pub fn with_id(mut self, value: u8) -> Profile {
        self.id = value;
        self
    }

    pub fn with_username(mut self, value: String) -> Profile {
        self.username = value;
        self
    }

    pub fn with_password(mut self, value: String) -> Profile {
        self.password = value;
        self
    }

    pub fn with_first_name(mut self, value: String) -> Profile {
        self.first_name = value;
        self
    }

    pub fn with_last_name(mut self, value: String) -> Profile {
        self.last_name = value;
        self
    }

    pub fn with_kind(mut self, value: Kind) -> Profile {
        self.kind = value;
        self
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Kind {
    #[serde(rename = "admin")]
    Admin,

    #[serde(rename = "teacher")]
    Mentor,

    #[serde(rename = "student")]
    Trainee,
}

impl Default for Kind {
    fn default() -> Self {
        Self::Trainee
    }
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
