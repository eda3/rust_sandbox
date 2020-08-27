fn main() {
  let x: u32 = {
    let mut b = String::new();
    std::io::stdin().read_line(&mut b).unwrap();
    b.trim().parse().unwrap()
  };
  println!("{}", x * x * x)
}