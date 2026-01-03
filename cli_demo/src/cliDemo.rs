pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    let command = args[1].clone();
    let name = "Swastik Acharyya";
    let status = "Single";

    println!("Args: {:?}", args);
    println!("Command: {}", command);

    if command == "hello" {
        println!("Hii {} , How are you??", name);
    } else if command == "bro" || command == "BRO" || command == "Bro" {
        println!("Heyy {} , Nigga ... What's up???", name);
    } else if command == "status" {
        println!("So you are {} , Really???", status);
    } else {
        println!("Invalid command");
    }
}
