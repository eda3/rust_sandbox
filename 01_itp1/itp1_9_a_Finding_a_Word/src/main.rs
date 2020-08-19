use std::io::*;
fn main(){
 let mut s=String::new();
 stdin().read_line(&mut s).unwrap();
 let w = s.trim().to_lowercase();
 let mut s=String::new();
 stdin().read_to_string(&mut s).unwrap();
 println!("{}",s.to_lowercase().split_whitespace().fold(0,|a,s|{if s==w{a+1}else{a}}))
}