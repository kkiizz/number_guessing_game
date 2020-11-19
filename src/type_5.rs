//乱数を使いたいので、randクレートを追加
//Cargo.tomlに rand="0.7.0" の追加を忘れずに
use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    //入力を受け取ってそのまま出力
    let mut st = String::new();

    //trim　を使って、改行コードを削除
    stdin().read_line(&mut st).expect("err");

    //parse　を使って、i32に変換
    //Result型がparseによって返されるので、unwrap()を使って中身を取り出す
    let num: i32 = st.trim().parse::<i32>().unwrap();

    //ランダムに1～10までの数字を取得
    let secret_num: i32 = rand::thread_rng().gen_range(1, 11);

    //if を使っていたものから、 matchに変更
    //&をつけて、参照を渡す
    //cmp　はOrderingの型を返す
    match num.cmp(&secret_num) {
        Ordering::Equal => println!("Equal!!!"),
        Ordering::Greater => println!("Too big..."),
        Ordering::Less => println!("Too small..."),
    }

    //ランダムに取得したnumを出力してみる
    println!("Secret_num is {}.", secret_num);
}