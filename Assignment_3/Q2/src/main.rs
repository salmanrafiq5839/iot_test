fn main() {
  let number = [1,2,3,4,5,6,7,8,9,10];
   let mut index = 0;
    while index <10
    {
    if (number[index] == 3 || number[index] == 7 || number[index] == 10 )
    {
   // println!(" : {}",number[index]);
   println!(" Special security check at {}",number[index]);
     
    }
    else 
    {
      println!("Number = {}",number[index]); 
    }
    index = index + 1;
    }
}
