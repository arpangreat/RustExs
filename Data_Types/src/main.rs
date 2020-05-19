mod DataTypes;
mod String;
mod Tuples;
mod Arrays;
mod Vectors;

fn main() {
    println!("Printing from DataTypes.rs");
    println!(" ");
    DataTypes::run();
    println!(" ");
    println!("Printing from String.rs");
    println!(" ");
    String::run();
    println!(" ");
    println!("Printing from Tuples.rs");
    Tuples::run();
    println!(" ");
    println!("Printing from Arrays.rs");
    Arrays::run();
    println!(" ");
    println!("Printing from Vectors.rs");
    Vectors::run();
}
