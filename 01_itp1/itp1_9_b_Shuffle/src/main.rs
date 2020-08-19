fn main(){
 loop{
  let mut w=r();
  if w=="-"{break};
  let m=r();
  let m=m.parse().unwrap();
  for _ in 0..m{
   let n=r();
   let n=n.parse().unwrap();
   w = format!("{}{}",&w[n..],&w[..n]);
  }
  println!("{}",w);
 }
}
fn r()->String{
 let mut b=String::new();
 std::io::stdin().read_line(&mut b).unwrap();
 b.trim().to_string()
}