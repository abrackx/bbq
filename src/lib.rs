extern crate generational_arena;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use bincode::serialize_into;
use generational_arena::Arena;

pub struct Database<'a> {
    pub name: String,
    pub path: &'a Path,
}

impl Database<'_> {
    pub fn create(&self) {
        let mut f = BufWriter::new(File::create(&self.path).unwrap());
        let mut arena = Arena::new();
        let _ = arena.insert(&self.name);
        serialize_into(&mut f, &arena).unwrap();
    }
}