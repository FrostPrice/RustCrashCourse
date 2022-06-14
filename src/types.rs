/*
Primitive Types--
    Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (Number of bits they take in memory)
    Floats: f32, f64
    Boolean (bool): true, false
    Characters (char): 'a', 'o'
    Tuples
    Arrays
*/

// Rust is a statically typed language, which means that it must know the types of all variables at compile time, however, the compiler can usually infer (determine) what type we want to use based on the value and how we use it.

pub fn run() {
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 4545454545544455445;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from Expression
    let is_greater = 10 < 5;

    let a1 = 'a';
    let face = '\u{1F600}';
    // let a1 = 'ab'; // Return an erros, char is a single character

    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}

// u: means unsigned, this will only have positive number. AKA: without sign -
// i: means interger, this type have the positive and negative numbers
// The larger the number of bits in the intergers, the more memory will consume, but you can use bigger numbers as their values

// f32 or f64: is a float type with decimals. The numbers after the f if the amount of bits the float will have, 64 is more precise and can handle larger number with more decimals, but costs more memory, than the 32
// PS: Decimals use . (dot)

// Boolean: is a true or false value

// Characters (char): is only one character based on Unicode, they are NOT String. To define a char use the '' (single quotes)

// Tuples: are lists

// Arrays: are a way to store multiple value of the same type. And in Rust they have a fixed lenth

//PS: Vectors are like arrays, but you can grow their size

// Defualt types:
// - For interger is i32
// - For floats is f64

// Explicit define type: add the let or const, then the name of the variable, after the name : (double dots), and finally the type of the variable

// You can get the max size of a type with the standard library, by using std::TYPE::MAX. (E.g: std::i32::MAX)
// The :: after the std means the item inside the std library

// You can also get a boolean from an Expression. Usually a comparisson return a boolean

// There's the possibility of using emojis on char

// PS: To use unicode, pass the syntax \u{CODE_FOR_THE_UNICODE}. \u initialize the unicode
