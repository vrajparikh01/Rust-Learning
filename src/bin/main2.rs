use std::{collections::HashMap, fmt::Display, sync::mpsc, thread};

fn main() {
    println!("Hello World 2!");

    // VECTORS
    println!("--------Vectors--------");
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    println!("{:?}", even_nos(&vec));
    println!("{:?}", vec);

    // Initialize a vector using macros
    let vec2 = vec![1, 2, 3, 4, 5];
    println!("{:?}", vec2);

    // HASHMAPS
    println!("--------Hashmaps--------");
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

    //returns an optional enum (what if blue not there in hashmap)
    let first_score = scores.get("Blue");

    match first_score {
        Some(score) => println!("Score: {}", score),
        None => println!("No score found for this team"),
    }

    let input_vec = vec![(String::from("Vraj"), 23), (String::from("Raj"), 30)];
    println!("{:?}", group_values_by_keys(input_vec));
    
    // ITERATORS
    println!("--------Iterators--------");
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("{}", val);
    }

    // how .iter works under the hood
    let mut v1_iter2 = v1.iter();
    while let Some(val) = v1_iter2.next() {
        print!("{}", val);
    }

    // mutable iterators
    let mut v2 = vec![1, 2, 3];
    let v2_iter = v2.iter_mut();
    for val in v2_iter {
        *val += 1;
    }
    println!("{:?}", v2);

    // Consuming adaptors
    let v3 = vec![1, 2, 3];
    let total: i32 = v3.iter().sum();
    println!("Total is: {}", total);
    // let sum2 = v3.iter().sum(); // this will give error as v3 is moved in above line

    // Iterators adaptors
    let v4 = vec![1, 2, 3];
    let v4_iter = v4.iter();
    let v4_iter2 = v4_iter.map(|x| x + 1);
    for val in v4_iter2 {
        println!("{}", val);
    }

    // Assignment: filter the odd numbers then double each value and create a new vector
    let v5 = vec![1, 2, 3, 4, 5];
    let v5_iter = v5.iter();
    let v5_iter2 = v5_iter.filter(|x| *x % 2 != 0).map(|x| x * 2);
    let v5_vec: Vec<i32> = v5_iter2.collect();
    println!("Assignment: {:?}", v5_vec);

    // STRING vs SLICE (&str)
    println!("--------String vs Slice--------");
    let mut s1 = String::from("Vraj");
    println!("{}", s1);

    s1.push_str(" Parikh");
    println!("{}", s1);

    s1.replace_range(5..s1.len(), "");
    println!("{}", s1);

    // Slices are references to a part of a string
    let s2 = &s1[0..5];
    println!("Slice: {}", s2);

    let v = vec![1, 2, 3];
    println!("{:?}", &v[1..2]);

    // String literals are slices but points directly to the binary
    let literal_string = "Hello, World!";
    println!("{}", literal_string);

    // GENERICS
    println!("--------Generics--------");
    let bigger = largest(1, 2);
    let bigger2 = largest("Vraj", "Parikh");
    println!("Bigger: {}", bigger);
    println!("Bigger2: {}", bigger2);

    // TRAITS
    println!("--------Traits--------");
    let user = User {
        name: String::from("Vraj"),
        age: 23,
    };
    // println!("{}", user.summarize());
    notify(user);

    // LIFETIMES
    println!("--------Lifetimes--------");
    let longest_str;

    let str1 = String::from("small");
    {
        let str2 = String::from("longest");
        longest_str = longest(&str1,&str2, "Generics, Traits, Lifetimes together");
        println!("{}", longest_str);
    }

    // STRUCT LIFETIME
    println!("--------Struct Lifetimes--------");
    let name = String::from("Vraj");
    let user2 = User2 {
        name: &name,
    };
    println!("{}", user2.name);

    // MULTITHREADING
    println!("--------Multithreading--------");

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {} from the spawned thread", i);
        }
    });

    for i in 1..50 {
        println!("Hi number {} from the main thread", i);
    }
    handle.join();

    let v10 = vec![1, 2, 3];
    let handle2 = thread::spawn(move || {
        println!("Moved {:?}", v10);
    });
    handle2.join();
}

struct User2<'a>{
    name: &'a str,
}

fn longest<'a, T>(first: &'a str, second: &'a str, ann: T) -> &'a str where T: Display {
    println!("Announcement: {}", ann);
    if first.len() > second.len() {
        return first
    } else {
        return second
    }
}

trait Summary {
    fn summarize(&self) -> String;
}

trait Fix {
    fn fix(&self) -> String {
        return String::from("Hi there from Fix trait");
    }
}

struct User {
    name: String,
    age: u32,
}

// implementing trait on the struct
impl Summary for User{
    fn summarize(&self) -> String {
        // format used to concatenate strings
        format!("Name: {}, Age: {}", self.name, self.age)
    }
}

// if no implementation fn is provided, default implementation is used from the trait
impl Fix for User {}

// Taking a trait as a parameter
// fn notify(item: impl Summary) {
//     println!("Breaking news! {}", item.summarize());
// }

// Using trait bounds to specify multiple traits
// taks generic type T which implements Summary and Fix
fn notify<T: Summary + Fix>(item: T) {
    println!("Breaking news! {}", item.summarize());
    println!("Fixed! {}", item.fix());
}

fn largest<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

fn even_nos(vec: &Vec<i32>) -> Vec<i32> {
    let mut even_vec = Vec::new();
    for i in vec {
        if i % 2 == 0 {
            even_vec.push(*i);
        }
    }
    even_vec
}

fn group_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for (key, value) in vec {
        hm.insert(key, value);
    }
    return hm;
}
