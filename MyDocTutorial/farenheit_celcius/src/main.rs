fn main() {
    // Taking User Input

    println!("Please input your number!!: ");

    let mut input = String::new();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failes to get the input");

    let input: f32 = input.trim().parse().expect("Please input a number");

    // Taking User Choice

    println!();
    println!("Enter 1 for farenheit converter or 2 for celcius converter!!!");
    println!("Please Enter your choice: ");

    let mut choice = String::new();

    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to Read your choice");

    let choice: u32 = choice.trim().parse().expect("Please Enter a number");

    if choice == 1 {
        farenheit_conv(input);
    } else if choice == 2 {
        celcius_conv(input);
    } else {
        println!("Please enter a valid choice");
    }
}

fn farenheit_conv(number: f32) {
    let res = ((9.0 / 5.0) * number) + 32.0;

    println!("The Result in Farenheit Scale: {}", res);
}

fn celcius_conv(number: f32) {
    let res = ((number - 32.0) / 9.0) * 5.0;

    println!("The Result in Celcius Scale: {}", res);
}
