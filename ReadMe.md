Rust Documentation

- Introduced by Mozilla Firefox to make FireFox much faster
- Companies using rust: Solana, Cloudfare, 1Password
- if you want to write very low level access to the system
- when you have to write languages where you need extremely low latency
- Smart contracts on Solana blockchain
- 2 step process: Build & optimize and then Run the code

Low Latency:
The time between an action and a system's response is minimal or there is almost no delay
Low latency is measured by "ping", which is the time it takes for data to travel between two points.
It's usually measured in milliseconds.
A latency of 30 milliseconds or less is often considered "very low latency".
Low latency is important for responsiveness in real-time applications like online gaming, video conferencing, and financial trading.

Phrases
- In Rust, if program compiles, it probably works
- You can't segfault if you don't have null (In computing, a segmentation fault (often shortened to segfault) or access violation is a fault, or failure condition, raised by hardware with memory protection, notifying an operating system (OS) the software has attempted to access a restricted area of memory (a memory access violation))
- Rust doesn't hide complexity from developers, it offers them right tools to manage all the complexities

Why Rust and not NodeJs?
1. JS doesn't care about the types, they don't enforce types anywhere (Typescript solves this problem)
Rust gives compilation error for mismatched types (even before running program, it tells you the error)
eg: let x = 1;
x = "Vraj":
console.log(x)
2. Rust is a systems language
    - You have access to lot of resources on the machine that you are running Rust on (not in JS)
    - get access to RAM (which is running and holding the variables) which means you can put variables here, remove variables, have references, etc.
    - runs very close to machine
    - Used for building compiler, browser, programs working close to OS
    - Rust is very fast compared to JS (if you want to build webrtc server, you will write in Rust)
    eg: mediasoup: C/Rust, pion: Golang. Mediasoup provides high level JS API to talk to C/Rust worker (just bcoz it's popular and ease of integration)
3. Generally faster
    - Rust has a separate compilation step (similar to C++) that spits out an optimised binary and does a lot of static analysis at compile time.
    - JS does JIT compilation...interpreting, compiling and running line by line, no separate compilation
    - If you want to build a trading bot where you are sending requests to place and execute multiple orders, it has to be really fast
    trading server will be written in Rust or Go or C or Zig or OCaml (JaneStreet)
    - In rust, code will build in binary and then run the binary (All compilation happens first and then running code fast)
    - There will be statical analysis in Rust where it will check if there are any error (will do bunch of optimisation and final binary will be very fast)
4. Concurrency
    - Running multiple threads on a single machine
    - If you run Node JS code, it will run on a single core of machine (single threaded)
    - Rust/Java/Go let's you spawn multiple threads and each thread can run independently on one of the cores
    - So, rust use complete power of machine
    - Each of these threads share the resources while in Node JS, we have to use IPC (Inter Process Communication) which is slow
5. Memory Safe
    - In C, you can manually allocate and deallocate resources on memory, you can have access to memory and ref/deref the variables
    - You can't do that in Rust which makes it memory safe
- Cargo is the package manager of Rust similar to npm
- Crates similar to packages
Cargo.toml => package.json
main.rs => index.js

Initialize project
- To initialise project, run "cargo init"
- By default, a rust application gets initialised. It means an end user app that actually executes independently. Spits out a binary when compiled
- cargo init --lib => This would initialise a library that you can deploy for other people to use
- You can see the binary file in "./target/debug/project"
- debug means you are running in local and building something
- you can also give cargo build --release, it won't be optimised build
- cargo build: rust => binary (compilation)
- ./target/debug/program => running the binary

Variables:
- You can define variables using the let keyword (very similar to JS)
- You can assign the type of the variable, or it can be inferred as well.
- By default, number reserve 32 bits in memory and decimal reserve 64 bits in memory
- All variables are immutable by default....if you want to change the variable value, add "mut", eg: let mut x:i32 = 5;
- Strings don't have fixed type

MEMORY MANAGEMENT:
- Whenever you run a program (C++, Rust, JS), it allocates and deallocates memory on the RAM.
Eg: function main() {
runLoop();
}

function runLoop() {
let x = [];
for (let i = 0; i < 100000; i++) {
x.push(1);
}
console.log(x);
}

As the runLoop function runs, a new array is pushed to RAM, and after runLoop() function is called, the garbage collector cleans the space and allocates it to other resource.

There are 3 types of memory management
1. Garbage Collector:
    - Can't do manual memory management
    - Usually no dangling pointers/memory issue
    - Written by smart people, developer doesn't have to worry about it
    - eg: Java, JS
2. Manual
    - You allocate and deallocate memory yourself
    - Can lead to dangling pointers/memory issue
    - Learning curve is high because developer has to do manual MM, trading firms use C because they want to maximise memory management
    - eg: C
3. Rust
    - Rust has its own ownership model for memory management so that the memory issues don't happen like dangling pointers, etc
    - Makes it extremely safe to memory errors

Memory management is a crucial aspect of programming in Rust, designed to ensure safety and efficiency without the need for a garbage collector.
Not having a garbage collector is one of the key reasons rust is so fast
It achieves this using the

- Mutability
- Heap and memory
- Ownership model
- Borrowing and references
- Lifetimes

Jargon #0: Mutability

- Immutable variables represent variables whose value cant be changed once assigned
- By default, all variables in Rust are immutable
- Immutable data is inherently thread-safe because if no thread can alter the data, then no synchronization is needed when data is accessed concurrently.
- No need to worry about compile-time checks, no race condition
- Knowing that certain data will not change allows the compiler to optimize code better.
- You can make variables mutable by using the mut keyword
- const in Javascript is not the same as immutable variables in rust. In JS, you can still update the contents of const arrays and objects

Jargon #1 - Stack vs heap

- If you don't have any data that grows or shrinks at the runtime the stack is great
- If you have growable/shrinkable data, just put predictable data on stack (ptr, len, capacity)
- Rust has clear rules about stack and heap data management:
- Stack: Fast allocation and deallocation. Rust uses the stack for most primitive data types and for data where the size is known at compile time (eg: numbers).
- What’s stored on the stack?: Numbers - i32, i64, f64..., Booleans - true, false..., Fixed sized arrays
- When a function is called, a stack frame is created and then variables are put in the frame (1 frame for 1 function)
- Heap: Used for data that can grow at runtime, such as vectors or strings or dynamic arrays
- Heap: store bunch of random data it is disorganised data and a variable whose length can change at runtime
- They are still stored on the stack but only the pointer to heap, length and capacity (how much extra space reserved)
- println!("Pointer: {:p}, Length: {}, Capacity: {}", s.as_ptr(), s.len(), s.capacity())
- If ptr gets changed in case string length increased then program will be slow because creating/copying things in heap is expensive

Jargon #2 - Ownership

- Ownership is a set of rules that govern how a Rust program manages memory. All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile. None of the features of ownership will slow down your program while it’s running.
- Rules: Each value in Rust has an owner, There can only be one owner at a time, When the owner goes out of scope, the value will be dropped.
- if the function is popped of the stack, all variables go away with it,
- Also, in the same function, you cannot access the variable out of the scope
- Heap variables always want to have a single owner, and if their owner goes out of scope, they get deallocated from the heap.
- At any time, each value can have a single owner. This is to avoid memory issues like Double free error and Dangling pointers.
- Eg: let s1 = String::from("Hello") and then let s2 = s1 then s2 will be the new owner of string in heap and s1 original owner will be invalid (s1 already moved to s2)
- Is there a better way to pass strings (or generally heap related data structures) to a function without passing over the ownership? YES (REFERENCES)

Jargon #3 - Borrowing and References

- References mean giving the address of a string rather than the ownership of the string over to a function
- You can transfer ownership of variables to fns. But by passing a reference to the string to the function take_ownership, the ownership of the string remains with the original variable, in the main function. This allows you to use my_string again after the function call.
- There can me many immutable references at the same time
- There can be only one mutable reference at a time
- If there is a mutable reference , you can’t have another immutable reference either.
- This to avoid any data races/inconsistent behaviour
- If someone makes an immutable reference , they don’t expect the value to change suddenly
- If more than one mutable references happen, there is a possibility of a data race and synchronisation issues

Structs:

- Structs in rust let you structure data together. Similar to objects in javascript
- struct User {
name: String,
age: u32,
email: String,
active: bool,
sign_in_count: u64,
}
Age, active and sign_in_count are stored in stack frame
name and email stored in heap (ptr, len, cap stored in stack)
- You can also implement structs , which means you can attach functions to instances of structs
- Very similar to classes in JS

Enums:

- Enums in rust are similar to enums in Typescript. They allow you to define a type by enumerating its possible variants
- Use enums rather than strings if you have less variants (bcoz you can pass anything in string and makes fn less strict)
- So this is much looser than strictly allowing only 4 variants for direction
- You can also use Enums with values (use pattern matching)
- The Option enum was introduced in Rust to handle the concept of nullability in a safe and expressive way.
- Unlike many programming languages that use a null or similar keyword to represent the absence of a value, Rust doesn't have null.
- pub enum Option<T> {
None,
Some(T),
}
Enum takes a generic type and return some type if there is some value or None if no value
- If you ever have a function that should return null, return an Option instead (might return value or might return null)

Collections:

- Things that dynamically increase/decrease in size that can store a lot of data together are called collections
- eg: arrays, vector, hashmaps, dynamically sized vectors, etc
- Rust’s standard library includes a number of very useful data structures called collections.
- Most other data types represent one specific value, but collections can contain multiple values.
- Data these collections point to is stored on the heap

Vectors:

- Vectors allows you to store more than one value in single data structure that puts all the values next to each other in memory.
- Vec<i32> is the default generic type of vectors
- We can print the whole vector by implementing the debug trait: println("{:?}", vec)

Hashmaps:

- Hashmaps store key-value pair in rust
- similar to objects in JS.
- Common methods: insert, remove, get, clear

Iterators:

- Iterator pattern allows you to perform some task on sequence of items in turn
- Responsible for iterating over each item and determining when the sequence has finished.
- In Rust, iterators are lazy, they have no effect until you call the methods that consume the iterators to use it up.
- Iterator is also a type(Iter<'_, i32>) in rust like vector, hashmaps and will return struct Iter
- Iterate over elements using:
1. Using for loops
2. Iterating after creating an iterator
- iter() method provides a way to iterate over the elements of a collection by borrowing them.
- You can't mutate the elements since we have an immutable reference to the internal elements. (Immutable iterator)
1. Iterating using .iter_mut() - if you want to borrow the mutable reference for each element
2. Iterator using into_iter()
- This iterator takes the ownership of the collection
- Useful when you no longer need the original collections
- Useful when you need the performance benefits by transferring ownership (avoiding references)

Iter: immutable reference to inner variables and don't want to transfer the ownership
IterMut: Mutable reference to inner variables and don't want to transfer the ownership
OntoIter: Move the variable into the iterator and don't want to use it later (transfer the ownership)

- If you write normal of loop without any iterator, it is similar to into_iter by default (takes ownership)

Consuming adapters:

- methods that call next are called consuming adapters
- calling them uses up iterator...can't use them again

Iterator adaptors:

- methods that don't consume the iterator, they produce diff iterator by changing some aspect of original iterator
- map, filter

Strings vs slices:

- String is a growable, mutable, owned and UTF-8 encoded string type
- Slices &str let you reference a contiguous sequence of elements in a collection rather than whole collection
- Slice is a kind of reference so it does not have ownership (It helps you view on the string)
- Slices can also be applied to other collections like arrays/vectors
- Third type of string is literal string (also an &str but it points directly to address in binary)

Generics:

- We use generics to create definitions for items like function signatures or structs, which we can then use with many different concrete data types.
- We place the generics in the signature of the function where we would usually specify the data types of the parameters and return value.
- Doing so makes our code more flexible and provides more functionality to callers of our function while preventing code duplication.

Traits:

- Similar to abstract classes in Java and interfaces in JS
- A trait defines the functionality of the particular type that can be shared with other types.
- We can use traits to define shared behaviour in an abstract way.
- We can use trait bounds to specify that generic type can be any type that has certain behaviour
- Like one class implements another abstract class in Java
- You can implement a function that takes an argument which implements the trait
- The impl trait syntax works for straightforward cases but it is actually sytax sugar for longer form called trait bound.

Lifetimes:

- The space where both arguments' lifetime is valid is where the return type will be valid (you are forcing that to the compiler for shorter lifetime)
- Means return type will be valid as long as both arguments are valid
- Lifetime of the return type will be intersection of the str1 and str2 arguments' lifetime
- <'a> Lifetime generic parameter
- fn longest<'a>(first: &'a str, second: &'a str) -> &'a str
- We want the signature to express the following constraint: the returned reference will be valid as long as both the parameters are valid. This is the relationship between lifetimes of the parameters and the return value.

Multithreading:

- In the most current operating systems, an executed program's code is run in a process and OS will manage multiple processes at once.
- Within program also, you can have independent parts that run simultaneously and features that run these are threads (which can run simultaneously on multiple cores)
- We’ll often use the move keyword with closures passed to thread::spawn because the closure will then take ownership of the values it uses from the environment, thus transferring ownership of those values from one thread to another.

Message passing:

- One increasingly popular approach to ensuring safe concurrency is message passing, where threads or actors communicate by sending each other messages containing data.
- Rust’s standard library provides an implementation of channels.
- A channel is a general programming concept by which data is sent from one thread to another.
- A channel has 2 halves: a transmitter and a receiver: One part of code calls the methods on transmitter with the data you want to send and another part checks the receiving end for arriving messages
- mpsc: multiple producer single consumer

Macros:

- Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming.
- Metaprogramming is useful for reducing the amount of code you have to write and maintain.