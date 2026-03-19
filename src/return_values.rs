fn get_value() -> i32 {
    42
}

// the placeholder @ {:?} specifies a debug value effectively
// printing the value with a @ "value" as a string
// this may be good for debugging

// Alternatively i could use the @ { } placeholder for it to print
// the value without the "" (string placeholder)

fn main() {
    let result = get_value();
    println!("Value: {:?}", result);
    vec();
}

// Using Vectors in Rust
// vec! generates code automatically before compilation
// ex: @ vec![10; 4] would generate [10, 10, 10, 10]
// effectivelly printing a vector
fn vec() {
    let random_vec = vec![1, 2, 3, 4, 5];
    let v = vec![10; 4];

    println!("Value {:?}", vec![10; 4]);
}
