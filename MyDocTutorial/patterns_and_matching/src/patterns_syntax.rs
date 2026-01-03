pub fn fun() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("Three"),
        _ => println!("anything"),
    }

    let a = Some(10);
    let b = 10;

    match a {
        Some(50) => println!("Got 50!"),
        Some(b) => println!("Matched, b = {:?}", b),
        _ => println!("Default case , a = {:?}", a),
    }

    println!("at the end: a = {:?}, b = {:?}", a, b);
}
