use std::collections::HashMap;

fn main() {
    let sentence = "the quick brown fox jumps over the lazy dog".to_string();

    // Use slicing to get the first three characters of the sentence
    // Note: Rust indices are byte-based. 0..3 would be "the".
    println!("{}", &sentence[0..3]);

    // concatenate using format!
    let description = format!("Title: Quick story\n{}", sentence);
    println!("{}", description);

    // Count occurrences of each vowel
    let mut vowel_counts = HashMap::new();
    for c in sentence.chars() {
        match c.to_ascii_lowercase() {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                *vowel_counts.entry(c).or_insert(0) += 1;
            },
            _ => continue,
        }
    }

    // Print the count for each vowel individually
    println!("Vowel counts:");
    for vowel in ['a', 'e', 'i', 'o', 'u'] {
        let count = vowel_counts.get(&vowel).unwrap_or(&0);
        println!("{}: {}", vowel, count);
    }

    // Split and collect into a vector
    // let words: Vec<&str> = sentence.split_whitespace().collect();
    let words = sentence.split(' ').collect::<Vec<&str>>();
    println!("{:?}", words);

    let reversed = sentence.chars().rev().collect::<String>();
    println!("{}", reversed);
    
    // Find and print the longest word
    let longest_word = find_longest_word(&sentence);
    println!("Longest word: {}", longest_word);
}

// Function that takes a sentence as input and returns the longest word
fn find_longest_word(sentence: &str) -> String {
    let mut longest_word = String::new();
    for word in sentence.split_whitespace() {
        if word.len() > longest_word.len() {
            longest_word = word.to_string();
        }
    }
    longest_word
}