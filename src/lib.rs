extern crate generational_arena;

use std::fs::{File, remove_file};
use std::io::BufWriter;
use std::path::Path;

use bincode::serialize_into;
use generational_arena::Arena;

pub struct Database {
    pub name: String,
}

impl Database {
    pub fn create(&self) {
        let mut f = BufWriter::new(File::create(&self.file_name()).unwrap());
        let mut arena = Arena::new();
        let _ = arena.insert(&self.name);
        serialize_into(&mut f, &arena).unwrap();
    }

    pub fn file_name(&self) -> String {
        format!("{}.bbq", &self.name)
    }
}