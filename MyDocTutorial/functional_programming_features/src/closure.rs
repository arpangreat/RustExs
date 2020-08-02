use std::thread;
use std::time::Duration;

pub fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

pub fn generate_workout(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("Calculating slowly ...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_closure(intensity));
        println!("Next, do {} situps!", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Today, Take a break! Remember to stay hidrated");
        } else {
            println!("Today, run for {} minuts", expensive_closure(intensity));
        }
    }
}
