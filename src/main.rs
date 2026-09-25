use std::fs;

struct Rect {
   width: u32,
   height: u32,
}

impl Rect {
    fn area(&self) -> u32 {
         self.width * self.height
    }
}

enum Shape {
    Circle(f64),  // Variant with associated data (radius)
    Square(f64),  // Variant with associated data (side length)
    Rectangle(f64, f64),  // Variant with associated data (width, height)
}

// Function to calculate area based on the shape
fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Square(side_length) => side_length * side_length,
        Shape::Rectangle(width, height) => width * height,
    }
}

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
    update_string();

    // Ownership
    let s1 = String::from("hello");
    let s2 = s1;
    // println!("{}", s1); // This line would cause a compile error because ownership has been moved.

    let my_string = String::from("hello");
    takes_ownership(my_string.clone()); // Pass a clone of the string to the function
    println!("{}", my_string); // This line would cause a compile error because ownership has been moved if clone was not used.

    // Borrowing and References
    let mut s1 = String::from("Hello");
    // let s2 = &mut s1; // compile error because you cannot have more than one mutable reference at a time
    update_word(&mut s1);
    println!("{}", s1);

    // There can be many immutable references to a variable, but only one mutable reference at a time.
    let s1 = String::from("Hello");
    let s2 = &s1;
    let s3 = &s1;
    println!("{}", s1);
    println!("{}", s2);
    println!("{}", s3);

    // Structs
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        active: true,
        username: String::from("Vraj"),
        email: String::from("vraj@gmail.com"),
        sign_in_count: 1,
    };
    println!("User 1 username: {:?}", user1.username);

    let rect = Rect {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {}", rect.area());

    // Enums and Pattern Matching
    let circle = Shape::Circle(5.0);
    let square = Shape::Square(4.0);
    let rectangle = Shape::Rectangle(3.0, 6.0);

    // Calculate and print the areas
    println!("Area of circle: {}", calculate_area(circle));
    println!("Area of square: {}", calculate_area(square));
    println!("Area of rectangle: {}", calculate_area(rectangle));

    // Error Handling
    let greeting_file_result = fs::read_to_string("hello.txt");

    match greeting_file_result {
        Ok(file_content) => {
            println!("File read successfully: {:?}", file_content);
        },
        Err(error) => {
            println!("Failed to read file: {:?}", error);
        }
    }

    // Option Enums
    let my_string = String::from("raman");
    match find_first_a(my_string) {
        Some(index) => println!("The letter 'a' is found at index: {}", index),
        None => println!("The letter 'a' is not found in the string."),
    }
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

fn update_string() {
    // Start with a base string on the heap
    let mut s = String::from("Initial string");
    println!("Before update: {}", s);
    println!("Capacity: {}, Length: {}, Pointer: {:?}", s.capacity(), s.len(), s.as_ptr());

    // Append some text to the string
    s.push_str(" and some additional text");
    println!("After update: {}", s);
    println!("Capacity: {}, Length: {}, Pointer: {:?}", s.capacity(), s.len(), s.as_ptr());
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string); // `some_string` now owns the data.
}

fn update_word(word: &mut String) {
    word.push_str(" World");
}

fn find_first_a(s: String) -> Option<i32> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index as i32);
        }
    }
    return None;
}