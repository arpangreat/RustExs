mod mut_refs;

fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);

    println!();

    println!("Printing from mut_refs");

    println!();

    mut_refs::run();
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
