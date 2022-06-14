// Conditionals - Used to check the condition of something and act on the result

pub fn run() {
    let age: i8 = 22;
    let check_id: bool = true;
    let know_person_of_age = true;

    // If/Else
    if age >= 21 && check_id || know_person_of_age {
        println!("Bartender: What would like to drink?");
    } else if age < 21 && check_id {
        println!("Bartender: Sorry, you have to leave");
    } else {
        println!("Bartender: I'll need to see your ID");
    }

    // Shorthande If
    let is_of_age = if age >= 21 { true } else { false };
    println!("Is Of Age: {}", is_of_age);
}

// Conditionals are basically is/is else/else statement

// In the if/else statement, when condition is true, then the block scope of that if will be run. If not, then it'll go to the next one. By the end when no condition is met, the else block will run by defualt

// You can use || (or), && (and), ! (not):
// On || (or) at least one condition needs to be true for the result to be true
// On && (and) all the conditions must to be true for the result to be true
// On ! (not) is the opposite of the current value. If it's true, then the ! will be false

// You can create a shorthand for the Rust If statement, when declaring a variable you can define the if/else statement as the variable value, so when the conditions match, the value of the variable will that result
