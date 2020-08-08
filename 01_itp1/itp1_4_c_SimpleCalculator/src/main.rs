use std::io::*;
fn main() {
 let s=std::io::stdin();
 for l in s.lock().lines(){
  let l = l.unwrap();
  let mut t=l.split_whitespace();
  let a=t.next().unwrap().parse::<i32>().unwrap();
  let b=t.next().unwrap().to_string();
  let c=t.next().unwrap().parse::<i32>().unwrap();
  let x=match &*b{
   "+"=>a+c,
   "-"=>a-c,
   "*"=>a*c,
   "/"=>a/c,
   _ => break
  };
  println!("{}",x);
 }
}