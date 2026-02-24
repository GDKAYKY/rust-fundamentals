use std::io;

pub fn generate_array() -> Vec<i32> {
    println!("Please insert the size of the array:");
    
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    let size: i32 = input
        .trim()
        .parse()
        .expect("Failed to parse size");
    
    let mut generated_array = Vec::new();
    for i in 0..size {
        generated_array.push(i);
    }
    
    generated_array
}
