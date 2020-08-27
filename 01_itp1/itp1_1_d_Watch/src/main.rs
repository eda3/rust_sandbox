fn main() {
  let s: i32 = {
    let mut b = String::new();
    std::io::stdin().read_line(&mut b).unwrap();
    b.trim().parse().unwrap()
  };
  println!("{}:{}:{}", s / 3600, s / 60 % 60, s % 60)
}