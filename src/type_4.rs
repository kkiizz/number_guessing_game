//乱数を使いたいので、randクレートを追加
//Cargo.tomlに rand="0.7.0" の追加を忘れずに
use rand::Rng;
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

    //if を使って、入力値とsecret_numを比較して、大小を出力
    if num == secret_num {
        println!("Equal!");
    } else if num < secret_num {
        println!("Too small...");
    } else {
        println!("Too big...");
    }

    //ランダムに取得したnumを出力してみる
    println!("Secret_num is {}.", secret_num);
}