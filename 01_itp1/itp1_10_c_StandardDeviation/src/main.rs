fn main(){
  loop{
    let v:Vec<f64>=r();
    if v[0]==0.{break}
    let a:Vec<f64>=r();
    let s=a.iter().fold(0.,|s,x|s+x);
    let b=(s)/(v[0]);
    println!("{}",(a.iter().map(|x|(x-b).powi(2)).fold(0.,|s,x|s+x)/v[0]).sqrt());
  }
}
fn r<T:std::str::FromStr>()->Vec<T>{
  let mut b=String::new();
  std::io::stdin().read_line(&mut b).unwrap();
  b.split_whitespace().flat_map(str::parse).collect()
}