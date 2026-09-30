use crate::utils::{bullet, code, header, info, note, remember, section};

const MAX_USERS: u32 = 1_000;
const APP_NAME: &str = "Rust Learning Guide";

pub fn constants() {
    header("CONSTANTS");

    introduction();
    declaration();
    naming();
    scope();
    immutability();
    constants_vs_variables();
    summary();
}

fn introduction() {
    section("What is a constant?");

    info("A constant is a value bound to a name that cannot be changed.");

    println!();

    code("const MAX_USERS: u32 = 1_000;");

    println!();

    bullet("Constants use the const keyword.");
    bullet("Constants must have an explicit type.");
    bullet("Constants cannot be changed.");
    bullet("Constants can be declared in module scope.");

    remember("A constant represents a value that is intended to remain constant.");
}

fn declaration() {
    section("Declaring constants");

    code("const MAX_USERS: u32 = 1_000;");
    code("const APP_NAME: &str = \"Rust Learning Guide\";");

    println!();

    info(&format!("MAX_USERS = {MAX_USERS}"));
    info(&format!("APP_NAME  = {APP_NAME}"));

    println!();

    info("The type annotation is required when declaring a constant.");

    note("Unlike let bindings, const declarations cannot rely on type inference.");
}

fn naming() {
    section("Constant naming");

    code("const MAX_USERS: u32 = 1_000;");
    code("const APP_NAME: &str = \"Rust Learning Guide\";");

    println!();

    bullet("Rust convention uses UPPER_SNAKE_CASE for constants.");
    bullet("Words are separated with underscores.");
    bullet("The name should describe the constant's meaning.");

    remember("Prefer clear UPPER_SNAKE_CASE names for constants.");
}

fn scope() {
    section("Constant scope");

    code("const MAX_USERS: u32 = 1_000;");

    println!();

    info("This constant is declared outside a function.");
    info("It can therefore be used by functions in this module.");

    println!();

    let local_limit: u32 = 50;

    code("const LOCAL_LIMIT: u32 = 50;");

    println!();

    info(&format!("Example value = {local_limit}"));

    println!();

    info("Constants can also be declared inside functions.");
}

fn immutability() {
    section("Constants cannot be changed");

    code("const MAX_USERS: u32 = 1_000;");

    println!();

    info(&format!("MAX_USERS = {MAX_USERS}"));

    println!();

    info("There is no mutable form of a constant.");
    info("You cannot assign a new value to a constant after declaring it.");

    note("Trying to modify a constant causes a compile-time error.");
}

fn constants_vs_variables() {
    section("Constants vs variables");

    println!("  Variable:");
    code("let age: u32 = 22;");

    println!();

    println!("  Mutable variable:");
    code("let mut age: u32 = 22;");
    code("age = 23;");

    println!();

    println!("  Constant:");
    code("const MAX_AGE: u32 = 120;");

    println!();

    bullet("let variables can be immutable or mutable.");
    bullet("const values are always immutable.");
    bullet("Constants require explicit types.");
    bullet("Constants are useful for values shared throughout a program.");

    remember("Use let for local changing program state; use const for fixed named values.");
}

fn summary() {
    section("Quick reference");

    bullet("const declares a constant.");
    bullet("Constants require an explicit type.");
    bullet("Constants cannot be mutated.");
    bullet("UPPER_SNAKE_CASE is the conventional naming style.");
    bullet("Constants can be declared outside functions.");
    bullet("Constants can be used wherever their scope allows.");

    println!();

    remember("const = named value that does not change.");
}
