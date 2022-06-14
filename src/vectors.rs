// Vectors - Resizable arrays

use std::mem;

// PS: A lot of methods used in Arrays can be used in Vectors
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4];

    // Re-assign value
    numbers[2] = 20;

    // Add on to vector
    numbers.push(5);
    numbers.push(6);

    // Pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get single value
    println!("Single Value: {}", numbers[0]);

    // Get Vector length
    println!("Vector length: {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);

    // Loop through Vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop & mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("Numbers Vec: {:?}", numbers);
}

// Vector works the exact same as Arrays, but with the key difference that Vectors can have it's values removed or added, meaning that will resize the Vector
// It's more common to use Vectors over Array

// Vector have values of the same element

// To define the Vector, for the type you must use the Vec<TYPE_OF_VALUES>. And for the values itself, you use the vec![], this will tell Rust to create a Vector and NOT an Array

// To add a value, you can use arr.push(value). This will add a value to the end of the Vector

// The arr.pop() method will remove the last element of the Vector

// And you can loop through Vectors

// The iter() method tells Rust that a Vector will be iterated (Looped)

// The iter_mut() method tells Rust that a Vector will be iterated (Looped) and it's values can be modified
