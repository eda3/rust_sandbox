// ITP1_2_A: Small, Large, or Equal
// http://judge.u-aizu.ac.jp/onlinejudge/description.jsp?id=ITP1_2_A&lang=ja

fn main() {
    let v: Vec<i32> = read();

    if v[0] < v[1] {
        println!("a < b");
    } else if v[0] > v[1] {
        println!("a > b");
    } else {
        println!("a == b");
    }
}

fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();
    return buf.split_whitespace().flat_map(str::parse).collect();
}