// Example demonstrating Rust's error handling with Result<T, E>
// Rust uses the Result enum to handle recoverable errors
// Result<T, E> has two variants: Ok(value) and Err(error)

// Define a function that returns a Result<f64, String>
// This means the function either returns Ok(f64) with a successful value
// or Err(String) with an error message
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        // Return an Err variant if division by zero would occur
        Err(String::from("Cannot divide by zero!"))
    } else {
        // Return an Ok variant with the result of the division
        Ok(a / b)
    }
}

fn main() {
    // Using match to handle the Result returned by our function
    let result = divide(10.0, 2.0);

    // Match forces us to consider both success and error cases
    match result{
        // If we get an Ok(value), extract the value inside
        Ok(value) => println!("Result: {}", value),
        // If we get an Err(error), handle the error case
        Err(error) => println!("Error occurred: {}", error),
    }

    // Another example with division by zero to trigger the error case
    let result = divide(10.0, 0.0);

    match result {
        Ok(value) => println!("Result: {}", value),
        Err(error) => println!("Error occurred: {}", error),
    }

    // Demonstrating Option<T> enum as well, which handles potentially absent values
    // Option<T> has two variants: Some(value) and None
    let numbers = vec![1, 2, 3, 4, 5];
    let index = 10; // This index is out of bounds for our vector

    // Vec::get() returns an Option<&T> - Some(&element) if the index exists, None otherwise
    match numbers.get(index) {
        // If the index exists, value will be a reference to the element (&i32)
        Some(value) => println!("Value at index {}: {}", index, value),
        // If the index doesn't exist, we get None and handle it gracefully
        None => println!("Index {} is out of bounds", index),
    }

    // Additional example of match with Option
    let valid_index = 2;
    match numbers.get(valid_index) {
        Some(value) => println!("Value at index {}: {}", valid_index, value),
        None => println!("Index {} is out of bounds", valid_index),
    }
}
