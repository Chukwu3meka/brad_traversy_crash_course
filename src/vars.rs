// Variables hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Brad";
    let mut age: u32 = 37;

    println!("My name is {} and I am {}", name, age);

    age = 38;

    println!("My name is {} and I am {}", name, age);

    // ? Define Constant
    const ID: u32 = 001;

    println!("ID: {}", ID);

    // ? Assign multiple vars
    let (my_name, my_age) = ("Brad", 37);

    println!("My name is {}, I'm {} years old", my_name, my_age)
}
