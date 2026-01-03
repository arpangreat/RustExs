// Primitive str = Immutable  fixed-length string somewhere in memory
// String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
 let mut hello = String::from("Hello ");

 // Get length
 println!("Length: {}", hello.len());

 // Push String
 hello.push_str("World!");

 // Capacity in bytes
 println!("Capacity: {}", hello.capacity());

 // Check if the String is empty
 println!("Is Empty: {}", hello.is_empty());

 // Contains
 println!("Contains 'World' {}", hello.contains("World"));

 // Replace
 println!("Replace: {}", hello.replace("World", "There"));

 // Loop through String by whitespace
 for word in hello.split_whitespace() {
     println!("{}", word);
 }

 // Create String with capacity
 let mut s = String::with_capacity(10);
 s.push('a');
 s.push('b');
 println!("{}",s);

 // Assertion testing
 assert_eq!(2, s.len());
 assert_eq!(10, s.capacity());




 println!("{}", hello);
}
