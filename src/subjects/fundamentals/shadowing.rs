use crate::utils::{bullet, code, header, info, note, remember, section};

pub fn shadowing() {
    header("SHADOWING");

    introduction();
    basic_shadowing();
    expressions();
    type_change();
    scope();
    shadowing_vs_mutability();
    summary();
}

fn introduction() {
    section("What is shadowing?");

    info("Shadowing happens when a new variable is declared using the same name");
    info("as an existing variable.");

    println!();

    code("let x = 5;");
    code("let x = x + 1;");

    println!();

    bullet("The second let creates a new binding.");
    bullet("The original binding is no longer the active binding for that name.");

    remember("Shadowing uses let again; assignment does not.");
}

fn basic_shadowing() {
    section("Basic shadowing");

    let x = 5;

    code("let x = 5;");

    println!();

    println!("  First x = {x}");

    let x = x + 1;

    code("let x = x + 1;");

    println!();

    println!("  Second x = {x}");

    remember("The second x shadows the first x.");
}

fn expressions() {
    section("Shadowing with expressions");

    let spaces = "   ";

    code("let spaces = \"   \";");

    println!();

    info(&format!("spaces before = \"{spaces}\""));

    let spaces = spaces.len();

    code("let spaces = spaces.len();");

    println!();

    info(&format!("spaces after  = {spaces}"));

    println!();

    bullet("The first spaces is a string slice.");
    bullet("The second spaces is a usize.");
    bullet("The new binding can use the previous binding's value.");

    remember("Shadowing can transform a value while keeping the same variable name.");
}

fn type_change() {
    section("Changing type through shadowing");

    let value = "42";

    code("let value = \"42\";");

    println!();

    info(&format!("value = {value}"));
    info("type = &str");

    let value: i32 = value.parse().expect("Failed to parse number");

    code("let value: i32 = value.parse().expect(\"Failed to parse number\");");

    println!();

    info(&format!("value = {value}"));
    info("type = i32");

    println!();

    bullet("Shadowing allows the new binding to have a different type.");
    bullet("The previous binding is replaced as the active name.");

    note("This is different from mut, because a mutable variable keeps the same type.");
}

fn scope() {
    section("Shadowing and scope");

    let x = 5;

    code("let x = 5;");

    println!();

    {
        let x = x * 2;

        code("let x = x * 2;");

        println!();
        info(&format!("Inside block: x = {x}"));
    }

    println!();

    info(&format!("Outside block: x = {x}"));

    println!();

    bullet("The inner x exists only inside the block.");
    bullet("The outer x is still available after the block.");

    remember("A variable declared inside a block has that block's scope.");
}

fn shadowing_vs_mutability() {
    section("Shadowing vs mutability");

    println!("  Shadowing:");

    code("let x = 5;");
    code("let x = x + 1;");

    println!();

    println!("  Mutability:");

    code("let mut x = 5;");
    code("x = x + 1;");

    println!();

    bullet("Shadowing creates a new binding.");
    bullet("mut allows the existing binding to be changed.");
    bullet("Shadowing can change the variable's type.");
    bullet("A mutable variable keeps its declared type.");

    remember("Same name does not mean same binding.");
}

fn summary() {
    section("Quick reference");

    bullet("Shadowing creates a new binding with the same name.");
    bullet("Shadowing uses let.");
    bullet("The new binding can have a different value.");
    bullet("The new binding can have a different type.");
    bullet("Shadowing can happen inside nested scopes.");
    bullet("Shadowing is different from mutability.");

    println!();

    remember("Think: let x again = create a new x.");
}
