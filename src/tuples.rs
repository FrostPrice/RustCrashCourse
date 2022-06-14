// Tuples group together values of different types
// Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("Brad", "Mass", 37);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}

// Tuples are a group of values that can be of different types,like a Object in JavaScript. But they have a max of 12 elements

// You must define the type of elements present in the order they'll be defined. When giving the type, after the : use the () separating values by collon ,

// Tuples are defined with the (value1, value2, value3, ..., value12)

// To access a tuple's value, use the values index, the index start in 0. And you must use dot notation for acessing the value. E.g: tuple.0, tuple.1
