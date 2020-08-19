fn main() {
 let v:Vec<usize>=z();
 let (r,c)=(v[0],v[1]);
 let mut s=vec![0;c+1];
 for _ in 0..r{
  let v:Vec<isize>=z();
  y(&v);
  let t=v.iter().sum::<isize>();
  println!(" {}",t);
  for (x,y) in v.iter().enumerate(){s[x]=s[x]+y}
  s[c]+=t;
 }
 y(&s);
 println!();
}
fn z<T:std::str::FromStr>()->Vec<T>{
 let mut b=String::new();
 let s=std::io::stdin();
 s.read_line(&mut b).unwrap();
 b.split_whitespace().flat_map(str::parse).collect()
}
fn y(v:&Vec<isize>)->(){
 print!("{}",v.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(" "));
}