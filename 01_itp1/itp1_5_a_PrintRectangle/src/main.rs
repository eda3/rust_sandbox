use std::io::BufRead;
fn main() {
 let s=std::io::stdin();
 for l in s.lock().lines(){
  let v:Vec<usize>=l.unwrap().split_whitespace()
   .map(|x|x.parse().unwrap())
   .collect();
  if v==[0,0]{break}
  println!("{}",("#".repeat(v[1])+"\n").repeat(v[0]));
 }
}