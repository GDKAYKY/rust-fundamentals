#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
}

// Implementing methods for the Person struct
impl Person {
    // Constructor function to create a new Person instance
    fn new(first_name: String, last_name: String, age: u8) -> Person {
        Person {
            first_name,
            last_name,
            age,
        }
    }

    // Method to get the full name of the person
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Method to update the age of the person
    fn set_age(&mut self, age: u8) {
        self.age = age;
    }

    // Method to check if the person is an adult (18 or older)
    fn is_adult(&self) -> bool {
        self.age >= 18
    }

    // Associated function (doesn't take &self as parameter)
    // Creates a baby with default values
    fn baby(name: String) -> Person {
        Person {
            first_name: name,
            last_name: String::from("Unknown"),
            age: 0,
        }
    }
}

// Example of a struct with multiple fields of different types
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    // Method to calculate the area of the rectangle
    fn area(&self) -> f64 {
        self.width * self.height
    }

    // Method to calculate the perimeter of the rectangle
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }

    // Method to check if the rectangle is actually a square
    fn is_square(&self) -> bool {
        (self.width - self.height).abs() < f64::EPSILON
    }

    // Associated function to create a square rectangle
    fn square(size: f64) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Example of a unit struct (no fields)
#[derive(Debug)]
struct Unit;

// Example of a tuple struct
struct Color(i32, i32, i32);

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
    fn deactivate(&mut self) {
        self.active = false;
    }
}

fn main() {
    let mut new_user = User::new(
        String::from("alfredodeza"),
        String::from("alfreodeza@example.com"),
        String::from("https://alfredodeza.com"),
    );
    println!("Hello, {}!", new_user.username);
    println!("Account {} status is: {}", new_user.username, new_user.active);
    new_user.deactivate();
    println!("Account {} status is: {}", new_user.username, new_user.active);

    // Creating a Person instance using the constructor
    let mut john = Person::new("John".to_string(), "Doe".to_string(), 25);

    println!("Original person: {:?}", john);
    println!("Full name: {}", john.full_name());
    println!("Is adult? {}", john.is_adult());

    // Updating the person's age
    john.set_age(16);
    println!(
        "After changing age to 16: {} - Is adult? {}",
        john.full_name(),
        john.is_adult()
    );

    // Creating a person using the associated function
    let baby = Person::baby("Little".to_string());
    println!("Baby: {:?}", baby);

    // Creating rectangles and demonstrating their methods
    let rect1 = Rectangle {
        width: 10.0,
        height: 5.0,
    };

    println!("\nRectangle: {:?}", rect1);
    println!("Area: {:.2}", rect1.area());
    println!("Perimeter: {:.2}", rect1.perimeter());
    println!("Is square? {}", rect1.is_square());

    // Creating a square using the associated function
    let square = Rectangle::square(4.0);
    println!("\nSquare: {:?}", square);
    println!("Area: {:.2}", square.area());
    println!("Is square? {}", square.is_square());

    // Demonstrating tuple struct
    let red = Color(255, 0, 0);
    println!("\nColor RGB: ({}, {}, {})", red.0, red.1, red.2);

    // Demonstrating unit struct
    let unit = Unit;
    println!("\nUnit struct: {:?}", unit);
}
