# rust-linecount
行を数えるプログラム。Rustの習作

# 詳細
標準入力からの行数を数えて標準出力に出力するプログラムです。

rust-ccに比べると、二つの点で異なっています。
- アルゴリズムを関数化したこと
- その関数をlib.rsに移したこと

# 関数化
line_count()関数は、`std::io::BufRead` トレイトを持つ型への参照を引数として受け取ります。
`std::io::BufReader`型構造体はBufRead トレイトを持っているため、main()からline_count()関数を呼ぶ
呼ぶときにBufReader構造体への参照を渡すことができます。

line_count()関数内部では特にエラー処理は行っていません。単に行数数えて返します。

main()関数は戻り値としてResult型を返しています。RustにおいてResult型はOk/Errの列挙値にエラーコードや
メッセージを紐付けることがありますが、ここでは単に空の値"()"を持たせています。

# 分割コンパイル
line_count()関数はmain.rsではなくlib.rsで定義しています。また、lib.rsの中ではalgorithmモジュールを
定義しており、そのモジュール内部がline_count()関数のスコープです。

lib.rsは特殊な名前のファイルであり、cargoはこのファイルをクレート・トップとして扱います。
そのため、lib.rsの中の名前を参照する際は、mainで次の文を使うべきではありません。

```Rust
mod lib;  // main.rsで使うべきではない。
```

lib.rsの中の名前（今回はモジュール名algorithm）を参照するときには、パッケージ名からの絶対パスを指定します。
```Rust
use rust_linecount::algorithm;
```
パッケージ名はcargo.tomlの中で宣言されています。このプロジェクトの場合、パッケージ名は"rust-linecount"ですが、
ハイフンはアンダースコアに変換されるためパッケージ名はrust_linecountになります。

# トレイトの可視性
lib.rsの中でBufRead トレイトをフルパスでuseしています。

```Rust
use std::io::BufRead;
```

トレイトを使う場合にはそれをどのようなパスで呼ぶ場合もフルパス形式でuse宣言しなければなりません。
具体的には、以下のいずれのパスで使う場合も、先のフルパスのuse文が必要です。


- `BufRead`
- `io::BufRead`
- `std::io::BufRead`

「[The Rust Programming Language日本語版](https://doc.rust-jp.rs/book-ja/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#%E6%85%A3%E4%BE%8B%E3%81%AB%E5%BE%93%E3%81%A3%E3%81%9Fuse%E3%83%91%E3%82%B9%E3%82%92%E4%BD%9C%E3%82%8B)」には次のように書いてあります。

> 構造体やenumその他の要素をuseで持ち込むときは、フルパスを書くのが慣例的です。 

慣例と書いてありますが、少なくともrust-linecountプログラムはフルパスで書かない限りコンパイルエラーを起こします。