// ITP1_1_C Rectangle
// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_1_C&lang=ja
use std::io;

// 空白区切りの数字列をコレクションデータ型にして返却
fn int_collect() -> Vec<u8> {
    let mut buf = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buf).unwrap();
    let iter = buf.split_whitespace().map(|s| s.parse::<u8>().unwrap());
    return iter.collect();
}


fn main() {
    // 読み込み
    let v: Vec<u8> = int_collect();

    // 図形の面積
    let area = v[0] * v[1];

    // 周の長さ
    let length = v[0] * 2 + v[1] * 2;

    // 図形の面積と周の長さを出力
    println!("{} {}", area, length);
}