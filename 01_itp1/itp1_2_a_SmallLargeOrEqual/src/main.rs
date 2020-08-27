fn main() {
  let mut b = String::new();
  std::io::stdin().read_line(&mut b).unwrap();
  let v: Vec<i32> = b.split_whitespace().flat_map(str::parse).collect();
  println!("a {} b", if v[0] < v[1] { "<" } else if v[0] > v[1] { ">" } else { "==" })
}