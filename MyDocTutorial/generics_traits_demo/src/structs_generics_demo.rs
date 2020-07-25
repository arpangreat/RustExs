struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &U {
        &self.y
    }
}

//pub fn run() {
fn main() {
    let integer = Point { x: 6, y: 7 };
    let float = Point { x: 9.0, y: 8.7 };
    let wont_work = Point { x: 8, y: 9.4 };

    println!("The x and y of integer is {} {}", integer.x(), integer.y());
    println!("The x and y of float is {:?} {:?}", float.x, float.y);
    println!(
        "The x and y of wont_work is {:?} {:?}",
        wont_work.x, wont_work.y
    );
}
