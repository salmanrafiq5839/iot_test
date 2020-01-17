fn main() {
 let student =(25,"A",80.3);
 
println!("Student no = {}",student.0);
println!("Student grade = {}",student.1);
println!("Student percantage = {}",student.2);
 
println!("Destructure"); // tuple
 let (x,y,z) = student;
println!(" {}",x);
println!(" {}",y);
println!(" {}",z);

 //Array 
 let no:[u32;5]=[20,10,50,100,10];
 println!(" {:#?}",no); //for vertical printing

 println!(" {:?}",no); // for horizontal printing

 //for same no array 
 let same_no =[5;3];
 println!(" {:#?}",same_no);

 // days array
 let day= ['M','T','W','T'];
 println!(" {:#?}",day); //print all
 println!(" {:#?}",day[1]); //print specific 
}
