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
