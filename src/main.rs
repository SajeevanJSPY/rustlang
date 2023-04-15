use rustlang_lexer::tokenize;
use std::fs;

const FILE_NAME: &'static str = "files/test.rs";

fn main() {
    let source_file = fs::read_to_string(FILE_NAME).unwrap();
    for token in tokenize(&source_file) {
        println!("{:?}", token);
    }
}
