use crate::utils::{bullet, code, header, info, remember, section};

pub fn arrays() {
    header("ARRAYS");

    introduction();
    declarations();
    access();
    mutation();
    length();
    types();
    summary();
}

fn introduction() {
    section("What is an array?");

    info("An array stores multiple values of the same type.");
    info("Its length is fixed at compile time.");

    println!();

    code("let numbers = [10, 20, 30, 40, 50];");

    println!();

    bullet("All elements must have the same type.");
    bullet("The number of elements is fixed.");

    remember("Array type includes both the element type and length.");
}

fn declarations() {
    section("Declaring arrays");

    let numbers = [10, 20, 30, 40, 50];
    let zeros = [0; 5];

    code("let numbers = [10, 20, 30, 40, 50];");
    code("let zeros = [0; 5];");

    println!();

    println!("  numbers = {:?}", numbers);
    println!("  zeros   = {:?}", zeros);

    println!();

    info("[value; length] repeats the same value a fixed number of times.");
}

fn access() {
    section("Accessing array elements");

    let numbers = [10, 20, 30, 40, 50];

    code("numbers[0]");
    code("numbers[2]");
    code("numbers[4]");

    println!();

    println!("  numbers[0] = {}", numbers[0]);
    println!("  numbers[2] = {}", numbers[2]);
    println!("  numbers[4] = {}", numbers[4]);

    println!();

    info("Array indexing starts at zero.");
}

fn mutation() {
    section("Mutable arrays");

    let mut numbers = [10, 20, 30];

    code("let mut numbers = [10, 20, 30];");
    code("numbers[1] = 99;");

    numbers[1] = 99;

    println!();

    println!("  numbers = {:?}", numbers);

    println!();

    info("The array must be declared with mut before its elements can be changed.");
}

fn length() {
    section("Array length");

    let numbers = [10, 20, 30, 40, 50];

    code("numbers.len()");

    println!();

    println!("  length = {}", numbers.len());

    println!();

    info("len() returns the number of elements in the array.");
}

fn types() {
    section("Array types");

    let numbers: [i32; 5] = [10, 20, 30, 40, 50];

    code("let numbers: [i32; 5] = [10, 20, 30, 40, 50];");

    println!();

    println!("  numbers = {:?}", numbers);

    println!();

    info("[i32; 5] means:");
    bullet("element type = i32");
    bullet("length = 5");

    remember("Array type syntax is [element_type; length].");
}

fn summary() {
    section("Quick reference");

    bullet("Arrays are compound types.");
    bullet("All elements have the same type.");
    bullet("Array length is fixed.");
    bullet("Indexing starts at zero.");
    bullet("Arrays can be mutable.");
    bullet("len() returns the number of elements.");

    println!();

    remember("Think: fixed-size collection of values with the same type.");
}
