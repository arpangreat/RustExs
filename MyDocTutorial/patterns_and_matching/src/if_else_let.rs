pub fn fun() {
    let favourite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favourite_color {
        println!("Using favourite color, {}, as the background!", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using Purple as Background");
        } else {
            println!("Using Orange as Background");
        }
    } else {
        println!("Using blue as Background");
    }
}
