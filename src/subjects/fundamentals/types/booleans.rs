use crate::utils::{bullet, code, header, info, remember, section};

pub fn booleans() {
    header("BOOLEAN");

    introduction();
    declarations();
    operations();
    comparisons();
    summary();
}

fn introduction() {
    section("What is a boolean?");

    info("A boolean represents one of two logical states:");

    println!();

    code("true");
    code("false");

    println!();

    remember("Rust's boolean type is bool.");
}

fn declarations() {
    section("Declaring booleans");

    let logged_in: bool = true;
    let admin: bool = false;

    code("let logged_in: bool = true;");
    code("let admin: bool = false;");

    println!();

    info(&format!("logged_in = {logged_in}"));
    info(&format!("admin     = {admin}"));
}

fn operations() {
    section("Logical operators");

    let a = true;
    let b = false;

    println!("Given:");
    println!("  a = {a}");
    println!("  b = {b}");

    println!();

    println!("  a && b → {}", a && b);
    println!("  a || b → {}", a || b);
    println!("  !a     → {}", !a);

    println!();

    bullet("&& = logical AND.");
    bullet("|| = logical OR.");
    bullet("!  = logical NOT.");
}

fn comparisons() {
    section("Booleans from comparisons");

    let age = 22;

    code("let age = 22;");
    code("age >= 18");

    println!();

    let adult = age >= 18;

    println!("  age >= 18 → {adult}");

    println!();

    info("Comparison operators produce bool values.");

    code("==  equal");
    code("!=  not equal");
    code(">   greater than");
    code("<   less than");
    code(">=  greater than or equal");
    code("<=  less than or equal");
}

fn summary() {
    section("Quick reference");

    bullet("bool has exactly two values: true and false.");
    bullet("&& combines conditions with AND.");
    bullet("|| combines conditions with OR.");
    bullet("! negates a boolean.");
    bullet("Comparisons produce bool values.");

    println!();

    remember("Booleans are the foundation of conditional logic.");
}
