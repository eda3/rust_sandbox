fn main() {
 let v:u64 = {
  let mut b = String::new();
  let s = std::io::stdin();
  s.read_line(&mut b).unwrap();
  b.trim().parse().unwrap()
 };
 let mut f;
 let mut c = 0;
 while c < v{
  f = true;
  c += 1;
  let mut x = c;
  if x % 3 == 0{
   print!(" {}", c);
   continue
  }
  while x != 0 && f{
   if x % 10 == 3 {
    print!(" {}", c);
    f = false;
   }
   x = x / 10;
  }
 }
 println!();
}