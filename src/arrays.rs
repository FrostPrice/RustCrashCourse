// Arrays - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 4] = [1, 2, 3, 4];

    // Re-assung value
    numbers[2] = 20;

    println!("{:?}", numbers);

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    let slice: &[i32] = &numbers[1..3];
    println!("Slice: {:?}", slice);
}

// Arrays have a fixed length, and the data type in an Array must be of the same type

// When creating an Array, you must define the type of the elements in the Array and the length of the same. The syntax goes as follows: let arr: [type, length] = [values....]

// Arrays are defined using the [] (square brackets)

// PS: The amount of elements in an Arrays must be equal as the specified length, meaning that it cannot be lower or higher

// You may get an specific value from the Array, to do that use [] after the name of the variable, and inside the [] is the index of the element. The index start in 0

// There is the possibility of changing the values of an Array, to the same type of value

// REMEMBER: You cannot add or remove element from an Array, only change them

// You can get the array length with the len() method

// You may check the amount of memory that the Array is occupying, to get access to that information, you must use the std library. E.g: std::mem::size_of_value(reference_to_the_value)

//// You can import a library and it's methods if you're using a lot of those in you're code. Just write the keyword use in the top of the code (being in the global scope of the file), and the path of the library. E.g: use std::mem::......; use std::io::........;

// Slices are used to get an interval from an Array. To get the values, simply use the arr[start_index..end_index]; and remember that the end_index is not inclusive by default, if you want to add it, before the end_index put a =, and the start_index is included
