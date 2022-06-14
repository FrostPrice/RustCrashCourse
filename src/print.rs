pub fn run() {
    // Print to console
    println!("Hello from the print.rs file");

    // Basic Formatting
    println!("{} is from {}", "Brad", "Mass");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Brad", "Mass", "code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity}",
        name = "John",
        activity = "Baseball"
    );

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for Debug Trait
    println!("{:?}", (12, true, "Hello"));

    // Basic Math
    println!("10 + 10 = {}", 10 + 10)
}

// pub makes the function public, so that it can be used in other files/scopes

// When using prinln!, you can add a {} to the string to create a template, and after the string, whatever value you pass as argument on the method, will replace the {} with the value
// Also the amount of {} you put on the string, is the amount of arguments you can pass

// You cannot print numbers directly on the string, you must use the {}

// You can use Positional Arguments if you hae repeated areas where the same text will be shown. All the arguments that you pass to replace the {}, will have the index, similar to one Array.
// REMEMBER: The first element index will start on 0, and the other will just follow along (1, 2, 3, 4, ...)

// And you may use Named Arguments, for better readibility. Simply add inside the {} the name of the argument, and when you're passing the argument, just declate the name of the Argument to the value you want. (E.g: "{test}", test = "NamedArgument")

// The Placeholder Traits are a way to define the way of the output should be, by converting the argument into a defined type, like binary.
// To use a Trait, you just need to add :TRAIT inside the {}, you must use a double collon, and the TRAIT will be replaced with the Trait identifier

// IMPORTANT, you can use the Debug Trait with :?
// And this trait mat accept multiples values at once, to do that simply add () with the values inside, separated with a , (comma)

// The () is called a Tuple

// You can pass expressions as arguments, and this will replace the {} (E.g: "{}", 10 + 10)
