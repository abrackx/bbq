use std::fs::{File, remove_file};
use std::path::Path;

use bbq;
use bbq::Database;

#[test]
pub fn should_create_db_file() {
    let setup = Setup {
        path: "test_db".into(),
    };
    let path = Path::new(&setup.path);
    let db = Database {
        name: "test".to_string(),
        path: path,
    };
    db.create();
    assert!(path.exists())
}

struct Setup {
    path: String,
}

impl Drop for Setup {
    fn drop(&mut self) {
        remove_file(&mut self.path).expect("could not remove")
    }
}