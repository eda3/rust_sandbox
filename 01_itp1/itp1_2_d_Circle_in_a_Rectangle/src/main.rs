fn main() {
 let v:Vec<i8>={
  let mut b=String::new();
  let s=std::io::stdin();
  s.read_line(&mut b).unwrap();
  b.split_whitespace().flat_map(str::parse).collect()
 };
 let (w,h,x,y,r) = (v[0],v[1],v[2],v[3],v[4]);
 println!("{}",if(0+r<=x&&x<=w-r)&&(0+r<=y&&y<=h-r){"Yes"}else{"No"});
}