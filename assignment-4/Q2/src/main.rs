fn main() {
    println!("Enter your integer value\t ");
    let mut integer = String::new();
    std::io::stdin().read_line(&mut integer).expect("Failed to read line");

    println!("Enter your float value \t ");
    let mut float = String::new();
    std::io::stdin().read_line(&mut float).expect("Failed to read line");

    println!("Enter your boolean value \t ");
    let mut boolean = String::new();
    std::io::stdin().read_line(&mut boolean).expect("Failed to read line");

    let int_integer: u32 = integer.trim().parse().expect("Please type a number!");
    let int_float: f32 = float.trim().parse().expect("Please type a number!");
    let int_boolean: bool = boolean.trim().parse().expect("Please type a number!");

 numbers(int_integer,int_float,int_boolean);

 }  
fn numbers(int_integer: u32,int_float: f32,int_boolean: bool)  {  
    println!("Your integer : {}",int_integer);
    println!("Your float : {}",int_float);
    println!("Your boolean is flase : {}",int_boolean);
}

