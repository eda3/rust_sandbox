fn main() {
    let mut v: Vec<i32> = read();
    v.sort();
    println!("{} {} {}", v[0], v[1], v[2]);
}

fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();
    buf.split_whitespace().flat_map(str::parse).collect()
}
