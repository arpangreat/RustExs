// Enums are types which have a few definite value

enum Movements {
    // Variants
    Up,
    Down,
    Right,
    Left,
}

fn move_avatar(m: Movements) {
    // Perform Action depending on info
    match m {
        Movements::Up => println!("Avatar moving up"),
        Movements::Down => println!("Avatar moving Down"),
        Movements::Right => println!("Avatar moving Right"),
        Movements::Left => println!("Avatar moving Left"),
    }
}

pub fn run() {
    let avatar1 = Movements::Up;
    let avatar2 = Movements::Down;
    let avatar3 = Movements::Right;
    let avatar4 = Movements::Left;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
