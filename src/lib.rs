pub mod algorithm {

    // traitを使うときには、trait自身を可視にしなければならない。
    use std::io;
    use std::io::BufRead;

    // traitの使用そのものは相対パスを使っても良い。
    pub fn line_count(reader: &mut impl io::BufRead) -> i32 {
        let mut counter = 0;

        // すべての行を列挙することで数える。
        for _line in reader.lines() {
            counter += 1;
        }
        // 返り値にセミコロンをつけてはいけない。
        counter
    }
}
