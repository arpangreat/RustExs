pub fn run() {
// Print to console
 println!("Hello printing from print");


 // Basic Formating
 println!("{} is from {}","Swastik","Rampurahat");


 // Positional Arguements
 println!("{0} is from {1} and {0} likes to {2}", "Swastik" , "Rampurhat" ,"Code");

 // Named Arguements
 println!("{name} likes to play {game}", name="Swastik", game="Cricket");


 // Placeholder traits
 println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

 // Placeholder for debug trait
 println!("{:?}", (12, true, "hello"));

 // Basic math
 println!("10 + 10 = {}", 10+10);
 
}
