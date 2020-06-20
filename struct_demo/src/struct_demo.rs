// Structs - Used to create custom DataTypes

// Traditional Struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple Struct
struct TColor(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct the Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_lastname(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to Tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}
pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 200;

    let mut tc = TColor(255, 56, 45);

    tc.1 = 89;

    println!("{} {} {}", c.red, c.green, c.blue);

    println!("Colors: {} {} {}", tc.0, tc.1, tc.2);

    let mut p = Person::new("Swastik", "Acharyya");

    println!("Person {} {}", p.first_name, p.last_name);
    println!("Person name: {}", p.full_name());
    p.set_lastname("Acharya");
    println!("Person name: {}", p.full_name());
    println!("Person Tuple: {:?}", p.to_tuple());
}
