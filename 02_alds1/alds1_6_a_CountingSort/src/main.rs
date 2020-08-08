fn main() {
    let v: Vec<usize> = r();
    let n = v[0];

    let a: Vec<u32> = r();
    let b = vec![0; n];
    k = max(a);
    let c = vec![0; k];
    println!("{:?}", b);

}

fn r<T:std::str::FromStr>()->Vec<T>{
    let mut b=String::new();
    let s=std::io::stdin();
    s.read_line(&mut b).unwrap();
    b.split_whitespace().flat_map(str::parse).collect()
}