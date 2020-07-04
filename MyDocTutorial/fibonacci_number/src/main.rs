fn main() {
    // Taking User Input
    println!("Please Enter your number");
    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to load Input");

    let input: u32 = input.trim().parse().expect("Please input a number");

    println!("Your result is: {}", fibonacci(input));
}

fn fibonacci(number: u32) -> u32 {
    if number == 0 || number == 1 {
        return number;
    }

    fibonacci(number - 1) + fibonacci(number - 2)
}
