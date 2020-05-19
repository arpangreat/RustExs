// Loops - Used to iterate until a condition is set

pub fn run() {
  let mut count = 0;
  // Infinte Loop
  //loop {
    // count += 1;
     // println!("Number: {}",count);

      // if count == 20 {
         //  break;
     // }
 // }

  // While Loop (FizzBuzz)
  //while count <= 100 {
    //  if count % 15 == 0 {
      //  println!("FizzBuzz");
     // } else if count % 3 == 0 {
          println!("Fizz");
     // } else if count % 5 == 0 {
          println!("Buzz");
     // } else {
     //     println!("{}", count);
     // }

      // Inc
    //  count += 1;
 // }

  // For Range
  for x in 0..101 {
      if x % 15 == 0 {
        println!("FizzBuzz");
      } else if x % 3 == 0 {
          println!("Fizz");
      } else if x % 5 == 0 {
          println!("Buzz");
      } else {
          println!("{}", x);

  }
 }
}
