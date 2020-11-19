//乱数を使いたいので、randクレートを追加
//Cargo.tomlに rand="0.7.0" の追加を忘れずに
use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;

fn main() {
    //loopを使って、"max_input_count"回までの入力を受け取り、判定するように改良
    //ランダムに取ってくる数の中で最大値を"max_random_num"に設定
    let max_input_count: i32 = 4;
    let max_random_num: i32 = 10;

    //ランダムに1～”max_random_num”までの数字を取得
    let secret_num: i32 = rand::thread_rng().gen_range(1, max_random_num+1);

    //let のままでは、変更を認められていないため、let mut
    //配列やタプル、vecなどとは違い、数値は
    let mut count: i32 = max_input_count;

    loop {
        //残りの入力回数を出力
        if 1 < count {
            println!("You can try {} more times!", count);
        } else {
            println!("The last challenge!!!");
        };

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
                println!("Equal!!! You win!!!\n");
                break;
            }
            Ordering::Greater => println!("Too big...\n"),
            Ordering::Less => println!("Too small...\n"),
        }

        //4回の入力と判定が終了した時点で負けを出力、breakでloopを抜ける
        count -= 1;
        if count <= 0 {
            println!("You lose...\n");
            break;
        }
    }
    //ランダムに取得したnumを出力してみる
    println!("Secret_num is {}.", secret_num);
}
