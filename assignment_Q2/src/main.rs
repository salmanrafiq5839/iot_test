fn main() {
    println!("LIST OF FRUITS ");
    let data  = ("apple", 2 , 150);
    let (fruit, weight , price) = data;
    println!("Fruit name : {}", fruit);
       println!("Weight in kg : {}", weight);
          println!("price per kg : {}", price);

    println!("Complete Tuple: {:?}", data);


}
