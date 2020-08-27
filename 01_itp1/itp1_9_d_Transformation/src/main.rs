fn main() {
  let s: Vec<String> = r();
  let mut s = s[0].to_string();
  let n: Vec<i32> = r();
  for _ in 0..n[0] {
    let v: Vec<String> = r();
    let c: &str = &v[0];
    let x: usize = v[1].parse().unwrap();
    let y: usize = v[2].parse().unwrap();
    match c {
      "print" => { println!("{}", &s[x..y + 1]); }
      "replace" => {
        let z: &str = &v[3];
        s.drain(x..y + 1);
        s.insert_str(x, z);
      }
      "reverse" => {
        let r = s.drain(x..y + 1).rev().collect::<String>();
        s.insert_str(x, &r);
      }
      _ => {}
    }
  }
}

fn r<T: std::str::FromStr>() -> Vec<T> {
  let mut b = String::new();
  std::io::stdin().read_line(&mut b).unwrap();
  b.split_whitespace().flat_map(str::parse).collect()
}