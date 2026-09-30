use crate::utils::{bullet, code, header, info, remember, section};

pub fn functions() {
    header("FUNCTIONS");

    introduction();
    declaration();
    parameters();
    return_values();
    expressions_as_return_values();
    scope();
    summary();
}

fn introduction() {
    section("What is a function?");

    info("A function is a named block of code that can be called to perform a task.");

    println!();

    code("fn greet() {");
    code("    println!(\"Hello!\");");
    code("}");

    println!();

    bullet("fn is used to define a function.");
    bullet("A function has a name.");
    bullet("The body contains the statements executed when it is called.");

    remember("Define reusable behavior once, then call the function whenever you need it.");
}

fn declaration() {
    section("Defining and calling functions");

    code("fn greet() {");
    code("    println!(\"Hello!\");");
    code("}");
    code("");
    code("greet();");

    println!();

    greet();

    println!();

    info("The function does not run when it is defined.");
    info("Its body runs when the function is called.");

    remember("A function call uses the function's name followed by parentheses.");
}

fn parameters() {
    section("Function parameters");

    code("fn greet(name: &str) {");
    code("    println!(\"Hello, {name}!\");");
    code("}");

    println!();

    greet_person("Youssef");
    greet_person("Rust");

    println!();

    bullet("Parameters are values received by a function.");
    bullet("Each parameter requires a type annotation.");
    bullet("Multiple parameters are separated by commas.");

    remember("Function parameters must have explicit types.");
}

fn return_values() {
    section("Return values");

    code("fn add(a: i32, b: i32) -> i32 {");
    code("    a + b");
    code("}");

    println!();

    let result = add(10, 20);

    println!("  add(10, 20) = {result}");

    println!();

    info("The -> syntax declares the return type.");
    info("The returned value must match that type.");

    remember("Functions can return a value to the code that called them.");
}

fn expressions_as_return_values() {
    section("Expressions as return values");

    code("fn square(number: i32) -> i32 {");
    code("    number * number");
    code("}");

    println!();

    let result = square(6);

    println!("  square(6) = {result}");

    println!();

    bullet("The final expression can be returned without a semicolon.");
    bullet("Adding a semicolon changes an expression into a statement.");

    remember("The last expression in a function body can be its return value.");
}

fn scope() {
    section("Function scope");

    let outside = 10;

    code("let outside = 10;");

    println!();

    show_scope(outside);

    println!();

    info("A function receives values through parameters.");
    info("Variables declared inside a function belong to that function's scope.");

    remember("Functions do not automatically access local variables from another function.");
}

fn summary() {
    section("Quick reference");

    bullet("fn defines a function.");
    bullet("Functions can receive parameters.");
    bullet("Parameters require explicit types.");
    bullet("-> declares a return type.");
    bullet("Functions can return values.");
    bullet("The final expression can be returned without a semicolon.");
    bullet("Functions have their own local scope.");

    println!();

    remember("Think: define behavior with fn, then call it by name.");
}

fn greet() {
    println!("  Hello from a function!");
}

fn greet_person(name: &str) {
    println!("  Hello, {name}!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn square(number: i32) -> i32 {
    number * number
}

fn show_scope(value: i32) {
    println!("  Function received value = {value}");
}
