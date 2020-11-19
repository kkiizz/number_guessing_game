use std::io::stdin;

fn main() {
    //入力を受け取ってそのまま出力
    let mut st = String::new();

    //trim　を使って、改行コードを削除
    stdin().read_line(&mut st).expect("err");
    let num = st.trim();
    println!("{:?}", num);
}