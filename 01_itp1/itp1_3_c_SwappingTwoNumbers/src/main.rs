fn main() {
 loop {
  let mut v:Vec<i32>={
   let mut b=String::new();
   let s=std::io::stdin();
   s.read_line(&mut b).unwrap();
   b.split_whitespace().flat_map(str::parse).collect()
  };
  if v==[0,0]{break;}
  v.sort();
  println!("{} {}", v[0],v[1]);
 }
}
