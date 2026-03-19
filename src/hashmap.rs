use std::collections::HashMap;

fn main() {
    let mut hashmap = HashMap::new();
    hashmap.insert("one", 1.to_string());
    hashmap.insert("two", 2.to_string());
    hashmap.insert("three", 3.to_string());
    println!("HashMap: {:?}", hashmap);

    for (key, value) in hashmap.iter() {
        println!("Key: {}, Value: {}", key, value);
    }
}