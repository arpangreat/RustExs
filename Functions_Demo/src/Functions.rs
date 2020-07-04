pub fn run() {
    greeting("Hello", "Swastik");

    let get_sum = add(12, 45);
    println!("Sum: {}", get_sum);

    // Closures
    let n3: i32 = 12;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(4, 87));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}
