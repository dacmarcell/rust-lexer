use std::{/* fs, */ env, fs};

fn main() {
    let file = env::args().nth(1).unwrap();
    let contents = fs::read_to_string(file).unwrap();
    println!("{:?}", contents);
}
