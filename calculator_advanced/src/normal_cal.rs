use std::io;

pub fn nor_cal() {
    let mut a = String::new();
    let mut b = String::new();
    let mut choice = String::new();

    fn nor_sum(a: f64, b: f64) -> f64 {
        a + b
    }

    fn nor_sub(a: f64, b: f64) -> f64 {
        a - b
    }

    fn nor_multi(a: f64, b: f64) -> f64 {
        a * b
    }

    fn nor_divi(a: f64, b: f64) -> f64 {
        a / b
    }

    println!("Please Enter your first Number:");

    io::stdin()
        .read_line(&mut a)
        .expect("Failed To read line!!");

    let a: f64 = a.trim().parse().expect("Please type a Number!!");

    println!("Please Enter your second Number:");

    io::stdin()
        .read_line(&mut b)
        .expect("Failed To read line!!");

    let b: f64 = b.trim().parse().expect("Please type a Number!!");

    println!("Enter 1 for sum, 2 for sub , 3 for multi , 4 for div");

    io::stdin()
        .read_line(&mut choice)
        .expect("Failed To read line!!");

    let choice: i32 = choice.trim().parse().expect("Please type a Number!!");

    if choice == 1 {
        println!("The Result is {}", nor_sum(a, b));
    } else if choice == 2 {
        println!("The Result is {}", nor_sub(a, b));
    } else if choice == 3 {
        println!("The Result is {}", nor_multi(a, b));
    } else if choice == 4 {
        println!("The Result is {}", nor_divi(a, b));
    } else {
        println!("Please input a right choice.");
    }
}
