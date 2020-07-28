fn main() {
    let v: Vec<i32> = read();

    if v[0] < v[1] && v[1] < v[2] {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn read<T: std::str::FromStr>() -> Vec<T> {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();
    return buf.split_whitespace().flat_map(str::parse).collect();
}