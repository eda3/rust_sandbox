fn main(){
 loop{
  let mut b=String::new();
  let s=std::io::stdin();
  s.read_line(&mut b).unwrap();
  if b.trim()=="0"{break}
  println!("{}",b.trim().chars().map(|c|c.to_digit(10).unwrap()).sum::<u32>());
 }
}