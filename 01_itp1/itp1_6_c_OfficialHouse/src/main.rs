fn main() {
 let mut a=vec![[[0;10];3];4];
 let n:Vec<isize>=r();
 for _ in 0..n[0]{
  let c:Vec<isize>=r();
  let b = (c[0]-1) as usize;
  let f = (c[1]-1) as usize;
  let r = (c[2]-1) as usize;
  let v = c[3];
  a[b][f][r]+=v;
 }
 let z:Vec<_> = a.iter()
  .flat_map(|x|x.iter())
  .collect();
 for (i, x) in z.iter().enumerate(){
   if i%3==0&&i!=0{
    println!("{}", "#".repeat(20));
   }
  println!(" {}", x.iter().map(std::string::ToString::to_string).collect::<Vec<_>>().join(" "));
 }
}
fn r<T:std::str::FromStr>()->Vec<T>{
 let mut b=String::new();
 let s=std::io::stdin();
 s.read_line(&mut b).unwrap();
 b.split_whitespace().flat_map(str::parse).collect()
}