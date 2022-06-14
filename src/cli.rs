use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Brad";
    let status = "100%";

    // println!("Args: {:?}", args);
    // println!("Command: {:?}", command);

    if command == "hello" {
        println!("Hi {}, how are you?", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("That is not a valid command")
    }
}

// std::env::args is able to get arguments when passed during the execution of cargo run
// The args.collect() will return a Vector, where the first element is always the target of the executible
