use std::io::{stdin, BufReader};

fn main() {
    let mut reader = BufReader::new(stdin());

    let counter = rust_linecount::line_count(&mut reader);

    println!("{}", counter);
}
