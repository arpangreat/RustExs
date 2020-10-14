mod advanced_traits_demo;
// use advanced_features;
mod advanced_types;
mod calling_function_in_same_name;
mod same_named_function;

fn main() {
    advanced_traits_demo::fun();
    println!("Calling from 'calling_function_in_same_name' module !!");
    println!("========================================================");
    println!(" ");
    calling_function_in_same_name::run();
    println!(" ");
    println!("Calling from 'same_named_function' module !!");
    println!("========================================================");
    println!(" ");
    same_named_function::run();
    println!(" ");
    println!("Calling from 'advanced_types' module !!");
    println!("========================================================");
    println!(" ");
    advanced_types::run();
}
