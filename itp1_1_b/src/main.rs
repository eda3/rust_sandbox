use std::io;

// 一行読み込み
fn input() -> u32 {
    let mut buf = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buf).unwrap();
    return buf.trim().parse().unwrap();
}

// 三乗して返却
fn cube(x: u32) -> u32 {
    return x * x * x;
}

fn main() {
    let x: u32 = input();
    println!("{}", cube(x));
}
