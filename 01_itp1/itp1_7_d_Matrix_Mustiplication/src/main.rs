fn main() {
 let v: Vec<usize>=r();
 let (x, y, z) = (v[0], v[1],v[2]);
 let a = s(v[0]);
 let b = s(v[1]);
 let mut c = vec![vec![0usize; z]; x];
 for k in 0..z{
  for i in 0..x{
   for j in 0..y{
    c[i][k] += a[i][j] * b[j][k]
   }
  }
 }
 for i in 0..c.len(){
  let mut d=c[i].len();
  for j in 0..d {
   if j != d-1{
    print!("{} ", c[i][j]);
   }else{
    println!("{}", c[i][j]);
   }
  }
 }
}
fn r<T:std::str::FromStr>()->Vec<T>{
 let mut b=String::new();
 let s=std::io::stdin();
 s.read_line(&mut b).unwrap();
 b.split_whitespace().flat_map(str::parse).collect()
}
fn s(i:usize)->Vec<Vec<usize>>{
 let mut a: Vec<Vec<_>> = vec![];
 for _ in 0..i{
  let w: Vec<usize>=r();
  a.push(w);
 }
 a
}