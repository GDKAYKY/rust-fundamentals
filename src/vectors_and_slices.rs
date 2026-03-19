fn main() {
    // VECTORS AND SLICES EXAMPLE

    // VECTORS
    // A Vector is a growable array stored on the heap
    // It's created with Vec<T> or vec! macro
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("Initial vector: {:?}", numbers);

    // Adding elements to a vector
    numbers.push(6);
    numbers.push(7);
    println!("After adding elements: {:?}", numbers);

    // Accessing elements in a vector
    let third_element = &numbers[2]; // Gets a reference to the 3rd element (0-indexed)
    println!("Third element: {}", third_element);

    // Safe way to access elements (returns Option<T>)
    // vec.get() returns an Option<&T> - either Some(&element) if index exists, or None if out of bounds
    // This is memory-safe as it prevents buffer overflows that could occur with unchecked access
    match numbers.get(10) {
        Some(value) => println!("Value at index 10: {}", value),
        None => println!("Index 10 is out of bounds"),
    }
    
    // Demonstrating unwrap with Option from vec.get()
    // When we use .get(index), Rust's type system ensures we handle potential out-of-bounds access
    // The return type is Option<&T>, enforcing explicit handling of the 'not found' case
    let valid_index = numbers.get(2).unwrap(); // This works because index 2 exists - returns &numbers[2]
    println!("Value at index 2 using get().unwrap(): {}", valid_index);
    
    // IMPORTANT: The difference between direct indexing and .get().unwrap():
    // numbers[10] would panic if index 10 doesn't exist
    // numbers.get(10).unwrap() would also panic, but it first returns None, then unwrap panics on None
    // So .get().unwrap() is safer when you're not sure the index exists, as you can handle the None case
    
    // Example of safe error handling with .get() instead of panicking with unwrap()
    let index_to_check = 10;
    if let Some(value) = numbers.get(index_to_check) {
        println!("Found value {} at index {}", value, index_to_check);
    } else {
        println!("Index {} is out of bounds, using default value", index_to_check);
        let default_value = 0;
    }
    
    // Demonstrating the ownership and borrowing aspects of .get()
    // .get() returns a borrowed reference (&T), not transferring ownership of the element
    // This follows Rust's borrowing rules - allowing access without moving the value
    let borrowed_element = numbers.get(0); // Type: Option<&i32>
    match borrowed_element {
        Some(ref_val) => println!("Borrowed value: {}", ref_val),
        None => println!("No value at index 0"),
    }
    // The original vector still owns the data and remains usable
    println!("Original vector still exists: {:?}", numbers);

    // Another example with Option - parsing strings to numbers
    let parsed_number: Option<i32> = "42".parse::<i32>().ok(); // ok() converts Result to Option
    match parsed_number {
        Some(n) => println!("Parsed number: {}", n),
        None => println!("Could not parse number"),
    }

    // Using unwrap on the parsed result (this will succeed)
    let successful_parse: Option<i32> = "100".parse::<i32>().ok();
    println!(
        "Successfully parsed with unwrap: {}",
        successful_parse.unwrap()
    );

    // Demonstrating unwrap with Result
    let result_success: Result<i32, _> = "75".parse();
    println!("Parsed with unwrap (success): {}", result_success.unwrap());

    // Removing elements
    let last_element = numbers.pop(); // Returns Option<T>
    println!("Removed last element: {:?}", last_element);
    println!("Vector after pop: {:?}", numbers);

    // Iterating over vector elements
    println!("Iterating over vector:");
    for num in &numbers {
        println!("Number: {}", num);
    }

    // SLICES
    // Slices are a view into a contiguous sequence of elements in a collection
    // They don't own the data, just reference a portion of it
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice_from_middle = &arr[2..6]; // Elements from index 2 to 5 (6 is excluded)
    println!("Slice from middle of array: {:?}", slice_from_middle);

    // Slicing a vector
    let vector_slice = &numbers[1..3]; // Creates a slice of the vector
    println!("Slice of vector [1..3]: {:?}", vector_slice);

    // String slices
    let text = String::from("Hello, world!");
    let hello = &text[0..5]; // String slice
    let world = &text[7..12];
    println!("First word: '{}', Second word: '{}'", hello, world);

    // Using slices as function parameters
    let data = vec![10, 20, 30, 40, 50];
    println!("Sum of all elements: {}", calculate_sum(&data)); // Pass slice to function
    println!("Sum of partial elements: {}", calculate_sum(&data[1..4])); // Pass slice directly

    // Demonstrating performance benefits of slices vs owned data
    let large_vector = (0..1000).collect::<Vec<i32>>();
    process_slice(&large_vector[..]); // Pass entire vector as slice without transferring ownership
}

// Function accepting a slice - works with both Vec<T> and arrays
fn calculate_sum(slice: &[i32]) -> i32 {
    // This function accepts any slice of i32 values
    // It doesn't take ownership, just borrows the data
    slice.iter().sum()
}

// Function demonstrating slice usage
fn process_slice(data: &[i32]) {
    println!(
        "Processing {} elements without taking ownership",
        data.len()
    );
    // Perform some processing on the data
    let sum: i32 = data.iter().sum();
    println!("Sum of slice: {}", sum);
}

// Example of common use cases for vectors vs slices
fn use_cases() {
    println!("\n--- USE CASES ---");

    // Use vectors when you need:
    println!("Vectors are ideal when you:");
    println!("  - Need to store an unknown number of elements at compile time");
    println!("  - Require adding or removing elements frequently");
    println!("  - Need to own the data");

    // Use slices when you need:
    println!("\nSlices are ideal when you:");
    println!("  - Want to borrow a portion of data without taking ownership");
    println!("  - Need to work with a fixed window of elements");
    println!("  - Want to write generic code that works with both arrays and vectors");
    println!("  - Performance is critical and you want to avoid copying");
}

// Demonstration of string-specific slices
fn string_examples() {
    println!("\n--- STRING EXAMPLES ---");

    let s = String::from("Rust fundamentals");

    // String slices are particularly useful for substring operations
    let rust_part = &s[0..4]; // "Rust"
    let fundamentals_part = &s[5..17]; // "fundamentals"

    println!("Original string: '{}'", s);
    println!("First part: '{}'", rust_part);
    println!("Second part: '{}'", fundamentals_part);

    // Working with string slices
    let words = split_sentence(&s);
    println!("Words in sentence: {:?}", words);
}

// Function that returns string slices from a string
fn split_sentence(text: &str) -> Vec<&str> {
    text.split_whitespace().collect()
}

// Run additional examples
fn additional_examples() {
    println!("\n--- ADDITIONAL EXAMPLES ---");

    // Creating vectors with different methods
    let v1: Vec<i32> = Vec::new(); // Empty vector
    let v2 = vec![0; 5]; // Vector with 5 zeros: [0, 0, 0, 0, 0]
    let v3 = (0..5).collect::<Vec<i32>>(); // Vector from iterator: [0, 1, 2, 3, 4]

    println!("Empty vector: {:?}", v1);
    println!("Vector of zeros: {:?}", v2);
    println!("Vector from range: {:?}", v3);

    // Working with mutable slices
    let mut numbers = vec![1, 2, 3, 4, 5];
    let slice = &mut numbers[1..4]; // Mutable slice

    for num in slice.iter_mut() {
        *num *= 10; // Modify values in the slice
    }

    println!("Modified vector: {:?}", numbers); // Original vector is changed
}
