// Array - Fixed list where elements are the same datatypes

use std::mem;

pub fn run() {
    let mut numbers: [i32; 4] = [1, 2, 3, 4];

    println!("{:?}", numbers);

    // Re-assign a value
    numbers[2] = 20;

    // Get single val
    println!("Single Value: {}", numbers[2]);

    // Get array Length
    println!("Array Length: {}", numbers.len());

    // Array are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // get slice
    let slice: &[i32] = &numbers[1..3];

    println!("Slice: {:?}", slice)
}
