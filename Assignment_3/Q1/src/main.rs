use std::io;

fn main() {
    println!("Enter your number : \t ");
    let mut no = String::new();
    io::stdin().read_line(&mut no).expect("Failed to read line");
   
    table(no);
    println!("Your your no : {}",no);

fn table(no:String)
{
    for no in 1..10{
println!( " {}" , no);
}
}
}
