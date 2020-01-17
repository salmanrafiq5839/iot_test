#[derive(Debug)]
struct Car {
    name:String,
    model :i32,
    maxspeed :i32
}

fn main() {
    let name = String::from("United Bravo");
    let model = 2019;
    let maxspeed = 130 ;

let car_1 = Car {
     name:String::from("United Bravo"),
     model : 2019,
     maxspeed : 130 
};
//print all instance
       println!("Struct  {:#?}",car_1);
// print all fields separately.
    println!("Car name: {}",car_1.name);
    println!("Model: {}",car_1.model);
    println!("Maxspeed: {}",car_1.maxspeed);

    let car_2 = Car {
        name:String::from("Toyota passo"),
        model : 2018,
        maxspeed : 160 
   };
   println!("Struct  {:#?}",car_2);
   
   let ret_student=car(name,model,maxspeed);
   println!("returned  {:?}",ret_student);
}

fn car (name:String,model:i32,maxspeed:i32) -> Car {
    println!("we are in car_2 instance");
    println!("{} {} {}",name,model,maxspeed);

    Car {
        name,
        model,
        maxspeed,
    }
}