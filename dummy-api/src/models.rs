use serde_derive::Serialize;
use serde_json::Value;
use super::store::Db;

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
    use std::error::Error;
    use bincode;

    pub const PROFILES: &str = "profiles";

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

    pub async fn initialize(db: super::Db, list: &[Profile]) {
        let mut db = db.lock().await;

        let docs: &mut Vec<Vec<u8>> = db.get_mut(PROFILES).unwrap();
        for profs in list {
            let data: Vec<u8> = bincode::serialize(&profs).unwrap();
            docs.push(data);
        }
    }

    pub async fn get_kind(db: super::Db, id: u8) -> Result<Kind, Box<dyn Error>> {
        let db = db.lock().await;

        let docs: &Vec<Vec<u8>> = db.get(PROFILES).unwrap();
        for data in docs.iter() {
            let prof: Profile = bincode::deserialize(&data).unwrap();
            if prof.id == id {
                return Ok(prof.kind.clone());
            }
        }

        Err("invalid role".into())
    }
}

pub mod course {
    use serde_derive::{Deserialize, Serialize};

    pub const COURSES: &str = "courses";

    #[derive(Default, Debug, Deserialize, Serialize, Clone)]
    pub struct Course {
        #[serde(default)]
        pub id: u8,

        #[serde(default)]
        pub title: String,

        #[serde(default)]
        pub description: String,

        #[serde(default)]
        pub creator_id: u8,
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

        pub fn with_creator_id(mut self, value: u8) -> Course {
            self.creator_id = value;
            self
        }
    }
}
