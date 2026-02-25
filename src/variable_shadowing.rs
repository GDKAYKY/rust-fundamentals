fn main(){
    let mut height = 190;
    height = height - 20;
    
    let result = if height < 180 {
        "Tall"
    } else if height > 170 {
        "Average"
    } else {
        "Short"
    };
    println!("Result: {}", result);

    let health = if height < 180 {"good"} else {"unknown"};
    println!("Health: {}", health);

    // shadowing to a different type
    let health = if height < 180 {true} else {false};   

    // This is definitely not a good idea, but it is possible to shadow a variable to a different type.
    // SO if you're going to do this, make sure you have a good reason to do so and that it doesn't make your code harder to read.
}
