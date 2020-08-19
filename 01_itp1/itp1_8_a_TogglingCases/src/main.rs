fn main() {
 let mut b=String::new();
 let s=std::io::stdin();
 s.read_line(&mut b).unwrap();
 let a = b.chars().map(|c|
  if c.is_lowercase(){
   c.to_uppercase().to_string()
  }else{
   c.to_lowercase().to_string()
  }
 ).collect::<String>();
 print!("{}", a);
}
