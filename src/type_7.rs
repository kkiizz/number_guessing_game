//乱数を使いたいので、randクレートを追加
//Cargo.tomlに rand="0.7.0" の追加を忘れずに
use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    //loopを使って、4回までの入力を受け取り、判定するように改良

    //ランダムに1～10までの数字を取得
    let secret_num: i32 = rand::thread_rng().gen_range(1, 11);

    //let のままでは、変更を認められていないため、let mutにする
    let mut count: i32 = 0;

    loop {
        //入力を受け取ってそのまま出力
        let mut st = String::new();

        //trim　を使って、改行コードを削除
        stdin().read_line(&mut st).expect("err");

        //parse　を使って、i32に変換
        //unwrap()だと、i32に変換できないものが入力されたらエラーが出るので、matchを使ったものに変更
        let num: i32 = match st.trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please input again!");
                //ここで continue にすることで、再度入力してもらう。（countは増やさなくて済む)
                continue;
            }
        };

        //&をつけて、参照を渡す
        //cmp　はOrderingの型を返す
        match num.cmp(&secret_num) {
            Ordering::Equal => {
                println!("Equal!!! You win!!!");
                break;
            }
            Ordering::Greater => println!("Too big..."),
            Ordering::Less => println!("Too small..."),
        }

        //4回の入力と判定が終了した時点で負けを出力、breakでloopを抜ける
        count += 1;
        if count >= 4 {
            println!("You lose...");
            break;
        }
    }
    //ランダムに取得したnumを出力してみる
    println!("Secret_num is {}.", secret_num);
}
