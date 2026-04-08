// Enums and Variants Example
//
// Enums (Enumerations) are a way to define a type by enumerating its possible variants.
// They are especially useful when a value could be one of several possible values.
// Rust's enums are similar to algebraic data types in functional languages.

// Define a simple enum with different variants
#[derive(Debug)]
enum Message {
    // Variant without data
    Quit,

    // Variant with anonymous struct-like fields
    Move { x: i32, y: i32 },

    // Variant with a single value
    Write(String),

    // Variant with multiple values of different types
    ChangeColor(i32, i32, i32),
}

// Define an enum for IP address versions
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// Another example of an enum with associated data
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// An enum for basic mathematical operations
#[derive(Debug)]
enum Operation {
    Add(i32, i32),
    Subtract(i32, i32),
    Multiply(i32, i32),
    Divide(i32, i32),
}

fn main() {
    // Creating instances of our enums
    let quit_message = Message::Quit;
    let move_message = Message::Move { x: 10, y: 20 };
    let write_message = Message::Write(String::from("Hello"));
    let color_message = Message::ChangeColor(255, 0, 128);

    println!("Messages:");
    println!("Quit: {:?}", quit_message);
    println!("Move: {:?}", move_message);
    println!("Write: {:?}", write_message);
    println!("ChangeColor: {:?}", color_message);

    // Calling the implementation method for each message
    impl_message(quit_message);
    impl_message(move_message);
    impl_message(write_message);
    impl_message(color_message);

    // Working with IP address enum
    println!("\nIP Addresses:");
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("Home: {:?}", home);
    println!("Loopback: {:?}", loopback);

    // Using enums with match expressions
    println!("\nUsing match expressions:");
    let operation = Operation::Add(10, 5);
    println!(
        "Operation: {:?}, Result: {}",
        operation,
        calculate_operation(&operation)
    );

    // Another example with Option<T> - one of the most commonly used enums in Rust
    println!("\nOption<T> examples:");
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    println!("Some number: {:?}", some_number);
    println!("No number: {:?}", no_number);

    // Using match with Option<T>
    match_option_example(some_number);
    match_option_example(no_number);

    // Demonstrating the power of enums with match
    println!("\nPattern matching with IP address:");
    print_ip_version(home);
    print_ip_version(loopback);
}

// Implementation of methods for handling messages
fn impl_message(msg: Message) {
    match msg {
        Message::Quit => println!("Quit message received"),
        Message::Move { x, y } => println!("Move to coordinates ({}, {})", x, y),
        Message::Write(text) => println!("Write text: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }
}

// Function to calculate the result of an operation
fn calculate_operation(op: &Operation) -> i32 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => {
            if *b != 0 {
                *a / *b
            } else {
                println!("Cannot divide by zero!");
                0
            }
        }
    }
}

// Function to handle Option<T> values
fn match_option_example(opt: Option<i32>) {
    match opt {
        Some(value) => println!("Got a value: {}", value),
        None => println!("Got nothing"),
    }
}

// Function to print IP address version
fn print_ip_version(ip: IpAddr) {
    match ip {
        IpAddr::V4(_, _, _, _) => println!("This is an IPv4 address"),
        IpAddr::V6(_) => println!("This is an IPv6 address"),
    }
}

// Example of using enums to represent game states
#[derive(Debug)]
enum GameState {
    Start,
    Playing {
        level: u32,
        score: u32,
    },
    Paused,
    GameOver {
        final_score: u32,
        level_reached: u32,
    },
}

fn handle_game_state(state: GameState) {
    match state {
        GameState::Start => println!("Game starting..."),
        GameState::Playing { level, score } => {
            println!("Currently playing level {} with score {}", level, score);
        }
        GameState::Paused => println!("Game paused"),
        GameState::GameOver {
            final_score,
            level_reached,
        } => {
            println!(
                "Game over! Final score: {}, Level reached: {}",
                final_score, level_reached
            );
        }
    }
}

// Enums vs Structs comparison
// While structs group related fields together, enums allow a value to be one of several possible variants
// This is powerful for modeling real-world scenarios where a value can only be one of a known set of possibilities

// The power of enums comes from pattern matching with `match` expressions
// Rust ensures that all possible variants are handled (exhaustiveness checking)
// This prevents runtime errors by forcing the programmer to consider all cases
