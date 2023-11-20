mod sub;
use std::io::{stdin, BufReader};

fn main() {
    let mut reader = BufReader::new(stdin());

    let counter = sub::line_count(&mut reader);

    println!("{}", counter);
}
