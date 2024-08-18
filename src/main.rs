use std::{/* fs, */ env, fs};

#[derive(Debug)]
struct Lexer {
    contents: String,
}

impl Lexer {
    pub fn new(contents: String) -> Self {
        Self { contents }
    }
}

fn main() {
    let file = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(file).unwrap();

    let lexer = Lexer::new(contents);

    println!("{:?}", lexer);
}
