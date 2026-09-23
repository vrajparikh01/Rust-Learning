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

    // Stack vs Heap
    stack_fn();
    heap_fn();
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

fn stack_fn() {
    // Declare a few integers on the stack
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Stack function: The sum of {} and {} is {}", a, b, c);
}

fn heap_fn() {
    // Create a string, which is allocated on the heap
    let s1 = String::from("Hello");
    let s2 = String::from("World");
    let combined = format!("{} {}", s1, s2);
    println!("Heap function: Combined string is '{}'", combined);
}