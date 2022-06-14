// Strucs - Used to create custom data types

// Traditional Struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// Tuple Struct
// struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construct Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to Tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    //// Traditional Struct
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };

    // c.red = 200;

    // println!("Color: {} {} {}", c.red, c.blue, c.green);

    //// Tuple Struct
    // let mut c = Color(255, 0, 0);

    // c.0 = 200;

    // println!("Color: {} {} {}", c.0, c.1, c.2)

    // Using Functions with Structs
    let mut p = Person::new("Mary", "Doe");
    println!("Person {}", p.full_name());
    p.set_last_name("Williams");
    println!("Person {}", p.full_name());
    println!("Person Tuple {:?}", p.to_tuple());
}

// Structs are like classes, you can define atributes, methods, etc.

// To define a Struct, use the struct keyword,then the name of the struct, and the block scope. They are defined in the global scope of the file

// PS: It is a convention to use UpperCase on the Struct name

// To use a Struct, just get the name of the Struct and use {}, inside the {} goes the values for the properties, assigning them like Javascript Objects

// And to access the values inside the Struct, use the dot syntax. E.g: struct.value1, struct.value2

// The Tuple Struct, works the same as the Traditional Struct, but the difference is that the Tuple Struct, works with the Index instead of the property name
// When assigning a Struct, instead of {} use (TYPE_1, TYPE_2, ....). And then use it like a normal tuple, by acessing values through index.

// REMEMBER: When using the Tuple Struct, you have the limitations of the Tuple, 12 max items

// To add functions in a Struct, you must use the impl keyword, then the name of the Struct, and the block scope, where all the functions will be defined
// The impl does not go inside the struct

// The to_string() method will convert a value a String, not a primitive string

// And to access function inside the Struct, simply get the Struct name and use the :: until you get the function you want. After all that, you can use the function as normal

// REMEMBER: When using the &self in a impl, you are making a reference to the current Struct

// The format! macro will return a String based on the values given. The first values is the final String format, and the later arguments are the values to be added in the first values. It works similar to the println! macro, but it doesn't print to the console, only returns the String

// You can modify the struct values. But when doing so, remember to add the &mut self, since you're changing the value of the current Struct
