fn main() {
    let s = input();
    let h = s / 3600;
    let m = (s - h * 3600) / 60;
    let s = s % 60;
    println!("{}:{}:{}", h, m, s);
}


fn input() -> u32 {
    let mut buf = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buf).unwrap();
    return buf.trim().parse().unwrap();
}
