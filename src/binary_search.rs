use crate::generate_array;

use rand::seq::IndexedRandom;
pub fn binary_search() {
    let array = generate_array::generate_array();
    println!("Generated array: {:?}", array);

    let mut sorted_array = array
        .into_iter()
        .filter(|&x| x % 2 == 0)
        .collect::<Vec<_>>();

    println!("Filtered array (even numbers): {:?}", sorted_array);

    sorted_array.sort();
    println!("Sorted array: {:?}", sorted_array);

    let mut rng = rand::rng();
    let objective = *sorted_array
        .choose(&mut rng)
        .expect("No even numbers in array");
    println!("\nSearching for: {}", objective);

    let target = objective;
    let mut left = 0;
    let mut right = sorted_array.len();

    while left < right {
        let mid = left + (right - left) / 2;
        println!("Checking index {} (value: {})", mid, sorted_array[mid]);

        if sorted_array[mid] == target {
            println!("\nFound {} at index {}", target, mid);
            return;
        } else if sorted_array[mid] < target {
            println!("  {} < {}, searching right half", sorted_array[mid], target);
            left = mid + 1;
        } else {
            println!("  {} > {}, searching left half", sorted_array[mid], target);
            right = mid;
        }
    }

    println!("\n{} not found in the array", target);
}
