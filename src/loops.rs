// Loops - Used to iterate util a condition is met

pub fn run() {
    let mut count = 0;

    // Infinite Loop
    // loop {
    //     count += 1;
    //     println!("Number: {}", count);

    //     if count == 20 {
    //         break;
    //     }
    // }

    // While Loop (FizzBuzz)
    // while count <= 100 {
    //     if count % 15 == 0 {
    //         println!("FizzBuzz");
    //     } else if count % 3 == 0 {
    //         println!("Fizz");
    //     } else if count % 5 == 0 {
    //         println!("Buzz");
    //     } else {
    //         println!("{}", count);
    //     }

    //     // Inc
    //     count += 1;
    // }

    // For Range
    for x in 0..100 {
        if x % 15 == 0 {
            println!("FizzBuzz");
        } else if x % 3 == 0 {
            println!("Fizz");
        } else if x % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", x);
        }
    }
}

// Loops will execute their block scope until a condition will return false

// In Rust there is an Infinite Loop, just use the loop keyword and the {}. Inside the {} goes the code that will run forever

// Use the break; keyword to exit a loop

// Rust has a while Loop, where the condition goes on top, after the while and between (). In this kind of loop just remember to add the count, so that the loop is finite

// The for ... in ... loop work by adding the condition and the counter on top of the Loop, after the for key word is the current value on the Loop, meaning the current iteration. The value after the in is the item to the looped, and the condition is the value itself, meaning that it will loop only what the item gives

// PS: You can define a range of numbers with start..end, the end is not inclusive, to add the end number, just put a = before it. E.g: 0..10
