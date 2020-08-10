use std::io::BufRead;
fn main() {
 let x = "#.".repeat(151);
 let s=std::io::stdin();
 for l in s.lock().lines(){
  let v:Vec<usize>=l.unwrap().split_whitespace()
   .map(|x|x.parse().unwrap())
   .collect();
  if v==[0,0]{break}
  for (a, b) in (0..2)
   .cycle()
   .map(|a|(a, a+v[1]))
   .take(v[0]){
    println!("{}",&x[a..b]);
  }
  println!();
 }
}