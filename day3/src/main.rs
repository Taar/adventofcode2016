extern crate day3;

use day3::{row, column};
use std::io::prelude::*;
use std::fs::File;

fn file_contents () -> String {
    let mut f = File::open("/tmp/input.txt").expect("No such file");
    let mut input = String::new();
    f.read_to_string(&mut input).unwrap();
    return input;
}

fn main() {
    let result = row::find_possible_triangles(file_contents());
    println!("Result: {}", result);
    let result_c = column::find_possible_triangles(file_contents());
    println!("Result: {}", result_c);
}
