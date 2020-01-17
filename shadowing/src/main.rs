fn main() 
{
    let mut x = 10;
    println!("x variable  {}",x);
   println!("x variable  {:p}",&x);
   let x = 20;
   println!("x variable  {}",x);
   println!("x variable  {:p}",&x);
// shadow
let x = 8; 
println!("{}",x);

let x = x * 2;
println!("{}",x);

let x:f64 =6.1;
println!("{}",x);
}
