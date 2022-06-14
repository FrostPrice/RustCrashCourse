// Variable hold primitive data or references to data
// Variables are immutable by default
// Rust is a block-scoped language

pub fn run() {
    let name = "Brad";
    let mut age = 37;

    println!("My name is {} and I am {}", name, age);

    age = 38;

    println!("My name is {} and I am {}", name, age);

    // Define Constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple variables at once
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}

// Variables are immutable by default. AKA: you cannot reassign data in a variable, you must tell rust that the variable is mutable with mut

// Rust is a block-scoped language. AKA: If you set a variable to a function, only inside that function you'll be able to access the variable

// REMEMBER: You cannot simply put the variable name in the println!, you must use the {}

// To make a variable mutable, simply add the mut before the variable name

// PS: If you assign a value to a variable, but never use that variable with that specific value, Rust will throw an error (value is never used)

// You may aslo define constant with the keyword const NAME_OF_CONSTANT: TYPE_OF_CONSTANT
// It's a common practice to have constant names as Upercase and with snake_camel.
// Also, you must define the type of the constant when creating it. To set a type, just put : (double dots) after the name of the constant and after that the type

// PS: Constants are not used so frequent

// You may assign multiples variables at once by using tuples or (). (E.g: let (var_1, var_2) = ("value_1", 2);)
