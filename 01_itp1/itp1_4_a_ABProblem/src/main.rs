fn main() {
 let v:Vec<i64>={
  let mut b=String::new();
  let s=std::io::stdin();
  s.read_line(&mut b).unwrap();
  b.split_whitespace().flat_map(str::parse).collect()
 };
 let (a,b)=(v[0],v[1]);
 println!("{} {} {:.6}",a/b,a%b,a as f64/b as f64);
}