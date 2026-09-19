fn main() {
    // Variables
    // Integer types
    let x = -5;
    let y: u32 = 1000;
    let z: f32 = 3.14;
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);

    // Boolean types
    let is_male = true;
    let is_above_18 = true;
    
    if is_male {
        println!("You are a male");

    } else {
        println!("You are not a male");
    }

    if is_male && is_above_18 {
        println!("You are a legal male");
    }

    // String types
    let greeting = String::from("Hello World");
    println!("{}", greeting);

    // Conditionals 
    let is_even = false;
    if is_even {
        println!("Variable is even");
    } else {
        println!("Variable is odd");
    }

    let first_name = get_first_name(String::from("Vraj Parikh"));
    println!("First name: {}", first_name);

    // Memory Management
    // Immutability
    let mut x: i32 = 1;
    x = 2; // No error
    println!("Mutable Variable: {}", x);
}

pub fn get_first_name(str: String) -> String {
    let mut first_name = String::from("");
    for c in str.chars() {
        if c == ' ' {
            break
        }
        first_name.push(c);
    }
    return first_name;
}