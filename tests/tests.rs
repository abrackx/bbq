use std::fs::{remove_dir_all, create_dir_all};
use std::path::Path;

use bbq;
use bbq::Database;
use std::env::set_current_dir;

#[test]
pub fn should_create_db_file() {
    let dir = "test_dir".to_owned();
    let db = Database {
        name: "test".to_owned(),
    };
    Setup::new(&dir);
    db.create();
    Setup::cleanup(&dir);
}

struct Setup {
    path: String,
}

impl Setup {
    pub fn new(path: &String) {
        create_dir_all(path);
        set_current_dir(path);
    }

    pub fn cleanup(path: &String) {
        set_current_dir("..");
        remove_dir_all(path).expect("Could not remove directory.")
    }
}