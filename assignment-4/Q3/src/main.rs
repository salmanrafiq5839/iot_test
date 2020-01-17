fn main() {
    let mut square :u32=0;
    // let number = (2,4,5);
    //println!("{:?}",number);

   printer(square);
}
fn printer(mut square :u32)  {  
    let number = (2,4,5);
    let mut counter :u8 = 0;
    println!("the number = {:?}",number);
    square = number.0 * number.0;
    println!("The square of number = {}",square);
    square = number.1 * number.1;
    println!("The square of number = {}",square);
    square = number.2 * number.2;
    println!("The square of number = {}",square);

    // loop {
    // square = number.0 * number.0;
    // println!("The square of number = {}",square);
    // counter = counter + 1;
    // if counter == 3{
    //     break
    //     }
    // }

}