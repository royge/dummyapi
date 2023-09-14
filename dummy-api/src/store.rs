use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

pub type Collection = HashMap<String, Vec<Vec<u8>>>;
pub type Db = Arc<Mutex<Collection>>;

pub async fn new_db(collections: Vec<&str>) -> Db {
    let db = Arc::new(Mutex::new(Collection::new()));

    let clone = db.clone();

    let mut _db = clone.lock().await;

    for name in collections {
        _db.entry(name.to_string()).or_insert(Vec::new());
    }

    return db
}

#[tokio::test]
async fn test_new_db() {
    let db = new_db(vec!["test"]);
    let db = db.await;
    let mut db = db.lock().await;
    assert_eq!(db.get("test"), Some(&vec![]));

    let docs: &mut Vec<Vec<u8>> = db.get_mut("test").unwrap();
    docs.push(vec![4]);
    assert_eq!(db.get("test"), Some(&vec![vec![4]]));
}
