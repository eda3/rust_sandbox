fn main() {
    let z: Vec<i32> = r();
    let ut v: Vec<i32> = r();
    println!("{}", v.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(" "));
}
fn r<T:std::str::FromStr>()->Vec<T>{
    let mut b=String::new();
    let s=std::io::stdin();
    s.read_line(&mut b).unwrap();
    b.split_whitespace().flat_map(str::parse).collect()
}