fn main() {
 let v:Vec<u32>={
  let mut b=String::new();
  let s=std::io::stdin();
  s.read_line(&mut b).unwrap();
  b.split_whitespace().flat_map(str::parse).collect()
 };
 println!("{}",(v[0]..v[1]+1).filter(|i|v[2]%i==0).count());
}