use std::io::stdin;

fn main() {
    //入力を受け取ってそのまま出力
    let mut st = String::new();
    stdin().read_line(&mut st).expect("err");
    println!("{:?}", st);
}