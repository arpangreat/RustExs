// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is block-scoped language


pub fn run(){
 let name = "Swastik";
 let mut age = 18;
 age = 19;
 println!("My name is {} and I am {}", name, age);


 // Define Constant
 const ID: i32 = 001;
 println!("ID: {}", ID);

 // Assign multiple vars
 let ( my_name, my_age ) = ("Swastik",19);
 println!("{} is {}", my_name, my_age);
}
