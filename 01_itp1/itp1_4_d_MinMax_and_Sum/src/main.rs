fn main() {
    let _z:Vec<i64>=r();
    let v:Vec<i64>=r();
    let min=v.iter().min().unwrap();
    let max=v.iter().max().unwrap();
    let sum=v.iter().fold(0, |sum, i|sum + i);
    println!("{} {} {}",min,max,sum);
}
fn r<T:std::str::FromStr>()->Vec<T>{
    let mut b=String::new();
    let s=std::io::stdin();
    s.read_line(&mut b).unwrap();
    b.split_whitespace().flat_map(str::parse).collect()
}