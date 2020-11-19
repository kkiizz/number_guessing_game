# Number guessing game
Rustに慣れるために、ちょっと書いてみたものです。[Rust ProgrammingLanguage](https://doc.rust-lang.org/book/title-page.html)の”Programming a Guessing Game"を参考に書いてみました。段階を踏みながらコードを書き、その過程を"type_0.rs"から"type_8.rs"に残しました。それぞれのファイルについて、前のファイルから何を追記，変更したのかを以下にまとめます。

# Information
## type_0.rs
- 入力値の受け取りと、出力
  - "std::io::stdin", "println!"
## type_1.rs
- 入力値からの改行コードを削除
  - "trim", "expect"
## type_2.rs
- 入力値のparse（Result型のまま）
  - "parse", "i32", "Result<T, Error>"
## type_3.rs
- Result型の中身を取り出す
  - "unwrap"
## type_4.rs
- 乱数の取得
- if を使った、入力値と取得した乱数の比較
  - "rand", "if"
## type_5.rs
- type_4.rs　のif によって判定している部分を、matchに変更
- Orderingによる判定
  - "match", "Ordering", ".cmp"
## type_6.rs
- 正しい入力値が得られているか判定
- Result型をmatchによって判定
- Errの場合は、とりあえず -1 を入れておく
  - "Result", "Ok", "Err", "match"
## type_7.rs
- loopを使って、任意の回数の入力値に対して判定
  - "loop", "let mut", "continue", "break"
## type_8.rs
- 乱数の範囲を定数により調整出来るように
- ついでに、入力回数も同様に変更
  - "ムーブ", "コピー", "let", "let mut", "/n"
## main.rs
- 微調整　完成

# 追記
このコードではtraitやstruct, iterator, implなどは使わなかったので、他の機会に試す予定です。
