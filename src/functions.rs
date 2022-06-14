// Functions - Used to store blocks of code for re-use

pub fn run() {
    greeting("Hello", "Jane");

    // Bind function value to variables
    let get_sum = add(5, 5);
    println!("Sum: {}", get_sum);

    // Closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C Sum: {}", add_nums(3, 3));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you!", greet, name);
}

fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

// To define a function, use the fn keyword, then the name of the function, after the name is (), and the block scope of that function

// Functions can have parameters, values that the function needs to do an operation. Parameters are defined in the (), and for each parameter you need to set their type
// When calling the function, the values passed into the parameters are called Arguments

// When returning a value on a Function, you must define the return type. This is done after the () with an arrow syntax, like: -> TYPE_OD_RETURN. E.g: -> usize

// REMEMBER: That after a function run, the return value can be stored in a Variable. So when calling a functio, you can set it to a variable

// You can explicity return a value with the return keyword. Or just leave the value without ; , this will let Rust know that it must return that value

// Closures are a shorthand for writing simple function. It can accept parameters too. After the = on a variable, add ||, and inside the || is the parameters and it's types, after the || goes the operation made on top of those paramters
// To use a closure, you just call them as regular functions. And closures aren't block scoped, you can use outside variables
