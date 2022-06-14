// Enums are types which have a few definite values

enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right,
}

fn move_avatar(m: Movement) {
    // Perform action depending on info
    match m {
        Movement::Up => println!("Avatar moving up"),
        Movement::Down => println!("Avatar moving donw"),
        Movement::Left => println!("Avatar moving left"),
        Movement::Right => println!("Avatar moving right"),
    }
}

pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}

// Enums are defined in the global scope of the file, and simply use the enums keyword, after that the name, and start the block scope

// PS: It is a common practice to use UpperCase with Enums Names

// Inside the Enum, there are the variants from that Enum.

// And you use the match statement to use the Enum Variants. The match statement works like a switch statement
// To use a match, write the keyword match them the value it will be doing the condiditions on, and start the block scope. For the conditions, first you validade if the value match with the condition, and them use a => (fat arrow), after the =>, goes what will be executed if the condition match

// If you are using an Enum, you need to validate all the possibilities in that match. And to do that get the Enum name :: and the name of the Variant. E.g: Enum::Variant
