use std::io;

fn main() {
    println!("Enter the Number in FLOAT datatype : \t ");
    let mut Number= String::new();
    io::stdin().read_line(&mut Number).expect("Failed to read line");

    println!("Your Number is  : {}",Number);

    let int_Number: f32 = Number.trim().parse().expect("Please type a number!");
    let mut no:f32 = 0.0;
    no= int_Number;
    printer(no);
    println!("thankyou for entering no... ");

fn printer(no: f32)  {           
    if no < 0.0
    {
        println!("Nmuber is Negative ... ");
    } else if no > 0.0
    {
        println!("Number is Positive ...");
    }else 
    {
        println!("Number is zero ...");
    }
}
}
