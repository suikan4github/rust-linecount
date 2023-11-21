use rust_linecount::algorithm;
use std::io::{stdin, BufReader};

fn main() {
    let mut reader = BufReader::new(stdin());

    let counter = algorithm::line_count(&mut reader);

    println!("{}", counter);
}
