// Reference Pointer - Point to a resource in memory

pub fn run() {
    // Primitive Array
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("Values: {:?}", (arr1, &arr2));

    // With Non-Primitives values, if you assign another variable to a piece of data, the first variable will no longer hold that value. You'll need to use a referece (&) to point to the resource

    // Vector (Non-Primitive)
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Vec Values: {:?}", (&vec1, vec2));
}

// With pointers, you can create a variable with a non-primiteve value, and point this value to another variable, so that both variables have the value.

// Since non-primitive values are able to change the block of memory they alocate, you must point to their reference

// To use a pointer, use the & before the name of the variable
