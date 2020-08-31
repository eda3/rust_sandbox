fn main(){
  let mut b=String::new();
  std::io::stdin().read_line(&mut b).unwrap();
  let v:Vec<f64>=b.split_whitespace().flat_map(str::parse).collect();
  let (a,b,c)=(v[0],v[1],v[2]);
  let h=b*c.to_radians().sin();
  let s=a*h/2.;
  let l=a+b+((a-b*c.to_radians().cos()).powi(2)+h.powi(2)).sqrt();
  println!("{}\n{}\n{}",s,l,h);
}