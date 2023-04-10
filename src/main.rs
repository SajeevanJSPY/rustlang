// Source Code ---> AST (Abstract Syntax Tree)

use std::fs;

const FILE: &'static str = "files/rust.rs";

fn main() {
    let source_file = fs::read_to_string(FILE).unwrap();
    println!("{:?}", source_file);
}
