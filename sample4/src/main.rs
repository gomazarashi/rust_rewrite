use std::io;

fn main() {
    println!("文字列を入力してください");
    let mut input_str = String::new();

    io::stdin()
        .read_line(&mut input_str)
        .expect("行の読み込みに失敗しました");
    println!("{}が入力されました",input_str);
}
