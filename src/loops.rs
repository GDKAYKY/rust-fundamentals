use rand::random;

fn main(){
    let mut x = 1;
// continue looping until x > 5
    loop {
        println! ("x is {}", x);
        x += 1;
        if x > 5 {
        break;
        }
    }
}

pub fn config(){
    struct Config {
        max_messages: i32,
    }
    // Example 1: Using Some() to wrap a value in an Option
    let max_messages = Some(42);
    
    // Example 2: Creating None variant of Option
    let timeout: Option<i32> = None;
    
    // Example 3: Using unwrap_or() to provide default value when Option is None
    println!("max_messages is {}", max_messages.unwrap_or(0)); // prints 42
    println!("timeout is {}", timeout.unwrap_or(30)); // prints 30 (default)
    
    // Example 4: Using match to handle both Some and None cases
    match max_messages {
        Some(value) => println!("We have {} max messages", value),
        None => println!("No max messages set"),
    }
    
    // Example 5: Using if let for cleaner Some handling
    if let Some(msg_count) = max_messages {
        println!("Message limit: {}", msg_count);
    }

    print!("max_messages is {}", max_messages.unwrap_or(0));
}

pub fn undefined_loop() {
    let  messages:i32= random::<i32>() % 15;

    loop { // This is an infinite loop and will continue until we break out of it
        if messages >= 10{
            print!("too many messages");
            break;
        }
        else if messages == 5 {
            println!("you only have 5 messages remaining");
            break;
        }
        else if messages <= 0 {
            println!("you don't have messages");
            break;
        }
        print!("you have {} messages", messages);
        break;
    }
}

pub fn conditional_loop() {
    let mut count = 0;

    while count < 5 {
        println!("Count is: {}", count);
        count += 1;
    }
}

pub fn while_loop() {
    let mut number = 10;

    while number >= 5 {
        println!("{}", number);
        number -= 1;
    }
}