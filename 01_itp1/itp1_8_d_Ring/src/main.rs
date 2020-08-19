fn main(){
 let s=r();
 let s=s.trim();
 let s=s.to_string()+&s;
 let x=r();
 let x=x.trim();
 println!("{}",if s.find(&x).is_some(){"Yes"}else{"No"})
}
fn r()->String{
 let mut b=String::new();
 std::io::stdin().read_line(&mut b).ok();
 b
}