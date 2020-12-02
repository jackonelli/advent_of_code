use itertools::Itertools;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file = "input/3/input";
    let mut file = File::open(file).expect("Opening file error");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Read to string error");
    let data = contents.trim().lines();
}
