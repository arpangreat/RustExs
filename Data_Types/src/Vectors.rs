// Vectors - Resizable Arrays

pub fn run() {
 let mut numbers: Vec<i32> = vec![1,2,3,4,5];

 // Re-assign value
 numbers[2] = 20;

 // Add on to Vector
 numbers.push(6);
 numbers.push(7);

 // Pop off last value
 numbers.pop();


 println!("{:?}", numbers);

 // Get single val
 println!("Single Value: {}", numbers[0]);

 // Get Array length
 println!("Vector Length: {}", numbers.len());

 // Arrays are stack allocated
 println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

 // Get Slices 
 let slice: &[i32] = &numbers[1..4];
 println!("Slice: {:?}", slice);

 // Loop through Vector values
 for x in numbers.iter() {
     println!("Numbers: {}", x);
 }

 // Loop & mutate values
 for x in numbers.iter_mut() {
  *x *= 2;
 }

 println!("Numbers Vec: {:?}", numbers);

}

