use std::io;

fn main() {
    println!("整数を入力してください");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("行の読み込みに失敗しました");
    let input: i32 = input.trim().parse().expect("整数を入力してください!");
    println!("{}が入力されました",input)
}
