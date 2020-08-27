fn main() {
  let s: i32 = {
    let mut b = String::new();
    std::io::stdin().read_line(&mut b).unwrap();
    b.trim().parse().unwrap()
  };
  let h = s / 3600;
  let m = (s - h * 3600) / 60;
  let s = s % 60;
  println!("{}:{}:{}", h, m, s);
}