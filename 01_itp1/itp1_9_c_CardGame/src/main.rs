use std::io::BufRead;
fn main(){
 let mut b=String::new();
 std::io::stdin().read_line(&mut b).unwrap();
 let n:usize=b.trim().parse().unwrap();
 let s=std::io::stdin();
 let t=s.lock().lines().take(n).fold((0,0),|a,line| {
  let line=line.unwrap();
  let mut l=line.split_whitespace();
  let s=l.next().unwrap();
  let t=l.next().unwrap();
  if s>t{(a.0+3, a.1)}else if s==t{(a.0+1,a.1+1)}else{(a.0, a.1+3) }
 });
 println!("{} {}",t.0,t.1);
}