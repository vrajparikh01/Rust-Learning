use std::{collections::HashMap};

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
