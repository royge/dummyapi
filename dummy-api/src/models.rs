use serde_derive::{Serialize};
use serde_json::Value;

#[derive(Serialize)]
pub struct Response {
    #[serde(skip_serializing_if = "Value::is_null")]
    pub data: Value,

    #[serde(skip_serializing_if = "Value::is_null")]
    pub error: Value,
}

pub mod profile {
    use rand::Rng;
    use serde_derive::{Deserialize, Serialize};
    use std::sync::Arc;
    use tokio::sync::Mutex;

    pub type Db = Arc<Mutex<Vec<Profile>>>;

    #[derive(Default, Debug, Deserialize, Serialize, Clone)]
    pub struct Profile {
        #[serde(default)]
        pub id: u8,

        pub username: String,
        pub password: String,

        #[serde(default)]
        pub first_name: String,

        #[serde(default)]
        pub last_name: String,

        #[serde(default)]
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

        pub fn with_generated_password(mut self) -> Profile {
            self.password = generate_password(8);
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

    fn generate_password(length: usize) -> String {
        const CHARSET: &[u8] =
            b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()";

        let mut rng = rand::thread_rng();
        let password: String = (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..CHARSET.len());
                CHARSET[idx] as char
            })
            .collect();

        password
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum Kind {
        #[serde(rename = "root")]
        Root,

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
}

pub mod course {
    use serde_derive::{Deserialize, Serialize};
    use std::sync::Arc;
    use tokio::sync::Mutex;

    pub type Db = Arc<Mutex<Vec<Course>>>;

    #[derive(Default, Debug, Deserialize, Serialize, Clone)]
    pub struct Course {
        #[serde(default)]
        pub id: u8,

        #[serde(default)]
        pub title: String,

        #[serde(default)]
        pub description: String,
    }

    impl Course {
        pub fn new() -> Course {
            Course::default()
        }

        pub fn with_id(mut self, value: u8) -> Course {
            self.id = value;
            self
        }

        pub fn with_title(mut self, value: String) -> Course {
            self.title = value;
            self
        }

        pub fn with_description(mut self, value: String) -> Course {
            self.description = value;
            self
        }
    }

    pub fn new_db() -> Db {
        Arc::new(Mutex::new(Vec::new()))
    }
}
