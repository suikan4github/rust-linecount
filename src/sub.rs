use std::io::BufRead;

pub fn line_count<R: BufRead>(reader: &mut R) -> i32 {
    let mut counter = 0;

    for _line in reader.lines() {
        counter += 1;
    }
    counter
}
