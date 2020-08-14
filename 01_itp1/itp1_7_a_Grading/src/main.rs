fn main() {
 loop {
  let v: Vec<i32>=r();
  if v[0] == -1 && v[1] == -1 && v[2] == -1 {
   break;
  }
  let r = v[0] + v[1];
  println!("{}", if v[0] == -1 || v[1] == -1 {"F"} else if 80 <= r {"A"} else if 65 <= r { "B" } else if 50 <= r { "C" } else if 30 <= r { if 50 <= v[2] { "C" } else { "D" } } else {"F"});
 }
}
fn r<T:std::str::FromStr>()->Vec<T>{
 let mut b=String::new();
 let s=std::io::stdin();
 s.read_line(&mut b).unwrap();
 b.split_whitespace().flat_map(str::parse).collect()
}
