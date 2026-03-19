/// Returns a String instance created from a string literal
/// This function demonstrates how to create a String and transfer ownership to the caller
fn string_literal() -> String {
    // Create a String from a string literal using the `from` method
    // This allocates memory on the heap and returns an owned String
    let s = String::from("I'm a String");
    // Return transfers ownership of the String to the calling function
    return s;
}

/// Takes a reference to a String and prints its value
/// This function borrows the String without taking ownership
/// The '&' indicates we're borrowing the value, not taking ownership
fn own_string(s: &String) {
    // Print the value of the string slice
    // Since we only have a reference, we don't own the data
    println!("{}", s);
}

fn main() {
    // Call the function and receive ownership of the returned String
    // The `real_string` variable now owns the String returned by string_literal()
    let real_string = string_literal();
    
    // Create another String instance directly
    let my_string = String::from("im proly maybe a string");

    // Pass a reference to the function so it can borrow the String
    // We use the '&' operator to pass a reference without transferring ownership
    // This way, `real_string` retains ownership and can still be used later
    own_string(&real_string);
    
    // At this point, `real_string` is still valid and owned by this scope
    // because we only passed a reference to `own_string()` function
}