fn main() {
    println!("Enter your student name\t ");
    let mut student = String::new();
    std::io::stdin().read_line(&mut student).expect("Failed to read line");

    println!("Enter your marks of subject 1 (max marks 100)\t ");
    let mut integer1 = String::new();
    std::io::stdin().read_line(&mut integer1).expect("Failed to read line");

    println!("Enter your marks of subject 2 (max marks 100)\t ");
    let mut integer2 = String::new();
    std::io::stdin().read_line(&mut integer2).expect("Failed to read line");

    let int_integer1: f32 = integer1.trim().parse().expect("Please type a number!");
    let int_integer2: f32 = integer2.trim().parse().expect("Please type a number!");
    
    let percantage =calculate(int_integer1,int_integer2);
    println!("Student Name : {}",student);
    println!("Your sub 1 : {}",int_integer1);
    println!("Your sub 2 : {}",int_integer2);

    if percantage >= 70.0
    {
     println!("student pass");
    println!("Your percantage : {}%",percantage);
    }
    if percantage < 70.0
    {
     println!("student Fail");
    println!("Your percantage : {}%",percantage);
    }

}
fn calculate(no:f32,no2:f32)->(f32)
{
let sum:f32 = no + no2;
let percantage = (sum/200.0)* 100.0;
(percantage)
}
