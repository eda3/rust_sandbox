fn main() {
 loop {
  let v: Vec<i32> = {
   let mut b = String::new();
   let s = std::io::stdin();
   s.read_line(&mut b).unwrap();
   b.split_whitespace().flat_map(str::parse).collect()
  };
  if v[0] == 0 && v[1] == 0 { break; }
  let n = v[0];
  let c = (1..n - 1).flat_map(|a| (a + 1..n).flat_map(move |b| (b + 1..n + 1).map(move |c| (a, b, c)))).filter(|&(a, b, c)| a + b + c == v[1]).count();
  println!("{}", c);
 }
}