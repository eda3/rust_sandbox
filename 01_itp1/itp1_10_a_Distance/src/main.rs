fn main(){
 let mut b=String::new();
 std::io::stdin().read_line(&mut b).unwrap();
 let v: Vec<f64>=b.split_whitespace().flat_map(str::parse).collect();
 println!("{}",((v[0]-v[2]).powi(2)+(v[1]-v[3]).powi(2)).sqrt());
}