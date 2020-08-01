fn main() {
 let mut i=input();
 let mut c=1;
 while i!=0{
  println!("Case {}: {}", c,i);
  i=input();
  c+=1
 }
}
fn input()->u32 {
 let mut b=String::new();
 let s=std::io::stdin();
 s.read_line(&mut b).unwrap();
 b.trim().parse().unwrap()
}
