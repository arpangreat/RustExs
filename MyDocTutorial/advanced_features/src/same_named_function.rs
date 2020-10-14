trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("Puppy")
    }
}

pub fn run() {
    println!("A baby is called a {:?}", Dog::baby_name());
    println!("A baby is called a {:?}", <Dog as Animal>::baby_name());
}
