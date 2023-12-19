// Vectors - are resizeable array

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    println!("{:?}", numbers);

    // Re-assign a value
    numbers[2] = 20;

    // Get single val
    println!("Single Value: {}", numbers[2]);

    // Get Vector Length
    println!("Vector Length: {}", numbers.len());

    // Vector are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[1..3];

    println!("Slice: {:?}", slice)
}
