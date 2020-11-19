use std::io::stdin;

fn main() {
    //入力を受け取ってそのまま出力
    let mut st = String::new();

    //trim　を使って、改行コードを削除
    stdin().read_line(&mut st).expect("err");

    //parse　を使って、i32に変換
    let num= st.trim().parse::<i32>();
    println!("{:?}", num);
}