// Arrays - Fixed list where elements are the same data types

pub fn run() {
 let mut numbers: [i32;5] = [1,2,3,4,5];

 // Re-assign value
 numbers[2] = 20;

 println!("{:?}", numbers);

 // Get single val
 println!("Single Value: {}", numbers[0]);

 // Get Array length
 println!("Array Length: {}", numbers.len());

 // Arrays are stack allocated
 println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

 // Get Slices 
 let slice: &[i32] = &numbers[1..4];
 println!("Slice: {:?}", slice);

}

