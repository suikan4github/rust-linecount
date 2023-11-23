// lib.rsにある algorithmモジュールへのアクセスは、パッケージからの絶対パスになる。
// パッケージ名はrust-linecountだが、useで参照するときにはrust_linecountになる。
use rust_linecount::algorithm;
use std::io;

fn main() -> Result<(), ()> {
    // stdinから読み込むBufReader型の変数を作る。
    // BufReaderはBufRead traitを持っているのでこの変数をline_count()に渡すことができる。
    let mut reader = io::BufReader::new(io::stdin());

    let counter = algorithm::line_count(&mut reader);

    println!("{}", counter);

    // このプログラムはOKしか返さない。
    Result::Ok(())
}
