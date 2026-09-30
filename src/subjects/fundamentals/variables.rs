use crate::utils::{bullet, code, header, info, note, remember, section};

pub fn variables() {
    header("VARIABLES & MUTABILITY");

    introduction();
    declarations();
    mutability();
    type_annotations();
    shadowing_difference();
    constants_difference();
    summary();
}

fn introduction() {
    section("What is a variable?");

    info("A variable gives a name to a value so the value can be used later.");
    info("Variables are immutable by default in Rust.");

    println!();

    code("let age = 22;");

    println!();

    bullet("let creates a variable binding.");
    bullet("The variable can then be used by its name.");
    bullet("Variables are immutable unless declared with mut.");

    remember("Rust variables are immutable by default.");
}

fn declarations() {
    section("Declaring variables");

    let name = "Youssef";
    let age = 22;

    code("let name = \"Youssef\";");
    code("let age = 22;");

    println!();

    info(&format!("name = {name}"));
    info(&format!("age  = {age}"));

    println!();

    info("Rust can usually infer the type of a variable from its value.");
}

fn mutability() {
    section("Mutable variables");

    let mut score = 10;

    code("let mut score = 10;");
    code("score = 20;");

    println!();

    println!("  Before: score = {score}");

    score = 20;

    println!("  After : score = {score}");

    println!();

    bullet("mut allows a variable's value to be changed.");
    bullet("Without mut, assigning a new value causes a compile-time error.");

    note("Mutability belongs to the variable binding, not to the value itself.");
}

fn type_annotations() {
    section("Type annotations");

    let age: i32 = 22;
    let price: f64 = 149.99;
    let active: bool = true;

    code("let age: i32 = 22;");
    code("let price: f64 = 149.99;");
    code("let active: bool = true;");

    println!();

    info(&format!("age    = {age}"));
    info(&format!("price  = {price}"));
    info(&format!("active = {active}"));

    println!();

    info("A type annotation explicitly tells Rust what type the variable should have.");
}

fn shadowing_difference() {
    section("Mutability vs shadowing");

    let mut mutable_value = 10;
    mutable_value = 20;

    let shadowed_value = 10;
    let shadowed_value = shadowed_value + 10;

    code("let mut value = 10;");
    code("value = 20;");

    println!();

    info(&format!("Mutable variable = {mutable_value}"));

    println!();

    code("let value = 10;");
    code("let value = value + 10;");

    println!();

    info(&format!("Shadowed variable = {shadowed_value}"));

    println!();

    bullet("mut changes the value stored in the same variable binding.");
    bullet("Shadowing creates a new binding with the same name.");

    remember("Mutability changes a value; shadowing creates a new binding.");
}

fn constants_difference() {
    section("Variables vs constants");

    code("let age = 22;");
    code("const MAX_USERS: u32 = 1000;");

    println!();

    bullet("Variables are declared with let.");
    bullet("Constants are declared with const.");
    bullet("Constants require an explicit type.");
    bullet("Constants cannot be changed.");
    bullet("Constants can be declared in many scopes, including module scope.");

    remember("Use a constant for a value that should never change.");
}

fn summary() {
    section("Quick reference");

    bullet("let creates a variable.");
    bullet("Variables are immutable by default.");
    bullet("let mut creates a mutable variable.");
    bullet("Rust can infer variable types.");
    bullet("Type annotations use : Type.");
    bullet("Variables and constants are different concepts.");
    bullet("Shadowing creates a new binding.");

    println!();

    remember("Start with let. Add mut only when the binding actually needs to change.");
}
