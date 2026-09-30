use crate::utils::{bullet, code, header, info, remember, section};

pub fn tuples() {
    header("TUPLES");

    introduction();
    declarations();
    access();
    destructuring();
    mixed_types();
    summary();
}

fn introduction() {
    section("What is a tuple?");

    info("A tuple groups multiple values into one compound value.");
    info("The values can have different types.");

    println!();

    code("let person = (\"Youssef\", 22, true);");

    remember("Tuples have a fixed number of elements.");
}

fn declarations() {
    section("Declaring tuples");

    let person = ("Youssef", 22, true);

    code("let person = (\"Youssef\", 22, true);");

    println!();

    println!("  person = {:?}", person);
}

fn access() {
    section("Accessing tuple elements");

    let person = ("Youssef", 22, true);

    code("person.0");
    code("person.1");
    code("person.2");

    println!();

    println!("  name    = {}", person.0);
    println!("  age     = {}", person.1);
    println!("  married = {}", person.2);

    println!();

    info("Tuple elements are accessed using a zero-based index.");
}

fn destructuring() {
    section("Destructuring");

    let person = ("Youssef", 22, true);

    code("let (name, age, married) = person;");

    let (name, age, married) = person;

    println!();

    println!("  name    = {name}");
    println!("  age     = {age}");
    println!("  married = {married}");

    println!();

    info("Destructuring assigns each tuple element to a variable.");
}

fn mixed_types() {
    section("Mixed types");

    let data: (&str, i32, f64, bool) = ("Rust", 2026, 3.14, true);

    code("(&str, i32, f64, bool)");

    println!();

    println!("  data = {:?}", data);

    println!();

    bullet("Tuple elements do not need to have the same type.");
    bullet("The tuple's structure and types are known at compile time.");

    remember("A tuple is useful when you need a small fixed group of related values.");
}

fn summary() {
    section("Quick reference");

    bullet("Tuples are compound types.");
    bullet("They have a fixed number of elements.");
    bullet("Elements can have different types.");
    bullet("Elements are accessed with .0, .1, .2, etc.");
    bullet("Tuples can be destructured.");

    println!();

    remember("Think: fixed-size group of potentially different types.");
}
