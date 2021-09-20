use std::env;
use std::fs;

fn main() {
    let filename = "content.txt";

    println!("Printing from {}", filename);

    let s = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("content of {}:\n' {} '", filename, s);
}
