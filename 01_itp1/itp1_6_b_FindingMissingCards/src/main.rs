use std::ops::Index;
use std::ptr::read_volatile;

fn main() {
    // トランプ一覧作成
    let s = "SHCD";
    let cards: Vec<_>= s
        .chars()
        .flat_map(|s|(1..14)
        .map(move|x|(s.to_string(), x.to_string())))
        .collect();

    // 除外カードリスト作成
    let n: Vec<i64>=r();
    let mut v :Vec<(String, String)>= vec!();
    for _ in 0..n[0]{
        let w: Vec<String>=r();
        v.push((w[0].clone(), w[1].clone()))
    }

    for c in cards{
        let d = c.clone();
        if !v.contains(&(d.0, d.1)){
            println!("{} {}", c.0, c.1);
        }
    }
}
fn r<T:std::str::FromStr>()->Vec<T> {
    let mut b = String::new();
    let s = std::io::stdin();
    s.read_line(&mut b).unwrap();
    b.split_whitespace().flat_map(str::parse).collect()
}
