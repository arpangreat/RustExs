mod for_concise_demo;
mod for_demo;
mod if_let_demo;
mod loop_demo;
mod type_annotations;
mod while_demo;

fn main() {
    let mut number = String::new();
    println!("1.type_annotations, 2.if_let_demo , 3.loop_demo , 4.while_demo , 5.for_demo , 6.for_concise_demo ");
    println!("Please Input Your Choice: ");
    std::io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: u32 = number.trim().parse().expect("Please Enter a number");

    if number == 1 {
        println!("From Types Annotations!!");
        println!();
        type_annotations::run();
    } else if number == 2 {
        println!("From if let demo!!");
        println!();
        if_let_demo::fun();
    } else if number == 3 {
        println!("From loop demo");
        println!();
        loop_demo::fun();
    } else if number == 4 {
        println!("From While Demo");
        println!();
        while_demo::fun();
    } else if number == 5 {
        println!("From For Demo");
        println!();
        for_demo::fun();
    } else if number == 6 {
        println!("From For Concise Demo");
        println!();
        for_concise_demo::concise();
    } else {
        println!("Please input a valid choice");
    }
}
