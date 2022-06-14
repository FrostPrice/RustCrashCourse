// Primitive String (str) = Immutable, fixed-length, string somewhere in memeory
// String = Growable, heap-allocated data structure - use when you need to modify our own string data

pub fn run() {
    // let hello = "Hello"; // str (Primitive String) // Cannot change this value later on the code
    let mut hello = String::from("Hello "); // String

    // Get length
    println!("Length: {}", hello.len());

    // Push char
    hello.push('W');

    // Push string
    hello.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Is Empty: {}", hello.is_empty());

    // Contains
    println!("Contains 'World': {}", hello.contains("World"));

    // Replace string
    println!("Replace: {}", hello.replace("World", "There"));

    // Loop trough string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);

    s.push('a');
    s.push('b');
    println!("{}", s);

    // println!("{}", hello);

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());
}

// With String you can use it as kind of an Array or Vector

// To create a str (Primitive String), simply use the "" (double quotes)

// Now to create a String, you use the String library and from the library get the from method, the argument passed into the method is the str (Primitive String). E.g: String::from("This is the best String")

// The len() method returns the length of the data. It works with almost every data type, including String and str (primitive string)

// There are 2 methods you can use for adding more text into a String
//      - push(str): used to add only a char, that is a single unicode character
//      - pushStr(str): used to add more than a single char. With this one, you're able to add another string into the existing string

// REMEMBER: To add the keyword mut when you want to make the variable mutable

// You can show the amount of bits the String is using with the capacity() method

// Also you may check is a string is empty with the method: is_empty() and it will return a Boolean

// To check if a string has a substring (compare 2 texts), use the contains(str) method

// The replace(str, str) method can change a subtring to another one. The first aegument is the string to be replaced, and the second one is the new value, that will replace the first one

// You can loop through a String with the for ... in ... loop

// The split_whitespace() will split the string by it's whitespace

// You may define a String with a certain capacity, so that the string size won't be bigger than the informed capacity. The syntax is as follows: String::with_capacity(usize)
// At fist the String is going to be empty, but you can push a value with the push() or pushStr() methods

//// Assertions
// Are used to compare values and only show an output if the test fails, this can be done using Rust built-in methods:
//      - assert_eq(value, value): this will say if the value of the left is the same as the value on the right
