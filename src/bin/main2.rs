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
