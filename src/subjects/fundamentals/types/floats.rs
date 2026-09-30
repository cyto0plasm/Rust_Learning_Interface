use crate::utils::{bullet, code, header, info, remember, section};

pub fn floats() {
    header("FLOATING-POINT NUMBERS");

    introduction();
    types();
    declarations();
    arithmetic();
    precision();
    methods();
    summary();
}

fn introduction() {
    section("What is a floating-point number?");

    info("Floating-point types represent numbers that can contain a fractional part.");

    println!();

    code("3.14");
    code("-0.5");
    code("100.0");

    println!();

    bullet("f32 = 32-bit floating-point number.");
    bullet("f64 = 64-bit floating-point number.");

    remember("Rust's default floating-point type is f64.");
}

fn types() {
    section("Floating-point types");

    println!("  Type      Bits      Bytes");
    println!("  ─────────────────────────");

    println!("  f32       32        {}", std::mem::size_of::<f32>());

    println!("  f64       64        {}", std::mem::size_of::<f64>());

    println!();

    info("f64 provides greater precision than f32.");
}

fn declarations() {
    section("Declaring floating-point values");

    let temperature: f32 = 36.5;
    let price: f64 = 149.99;

    code("let temperature: f32 = 36.5;");
    code("let price: f64 = 149.99;");

    println!();

    info(&format!("temperature = {temperature}"));
    info(&format!("price = {price}"));
}

fn arithmetic() {
    section("Arithmetic");

    let a: f64 = 10.5;
    let b: f64 = 2.0;

    println!("Given:");
    println!("  a = {a}");
    println!("  b = {b}");

    println!();

    println!("  Addition       : {}", a + b);
    println!("  Subtraction    : {}", a - b);
    println!("  Multiplication : {}", a * b);
    println!("  Division       : {}", a / b);
}

fn precision() {
    section("Floating-point precision");

    let result = 0.1_f64 + 0.2_f64;

    code("let result = 0.1_f64 + 0.2_f64;");

    println!();

    println!("  result = {result}");

    println!();

    info("Floating-point numbers use binary representation.");
    info("Some decimal fractions cannot be represented exactly in binary.");

    remember("Do not assume floating-point arithmetic is exact.");
}

fn methods() {
    section("Useful floating-point methods");

    let number = -3.7_f64;

    println!("Given:");
    println!("  number = {number}");

    println!();

    println!("  abs()   → {}", number.abs());
    println!("  floor() → {}", number.floor());
    println!("  ceil()  → {}", number.ceil());
    println!("  round() → {}", number.round());
    println!("  trunc() → {}", number.trunc());

    println!();

    bullet("abs() → absolute value.");
    bullet("floor() → rounds toward negative infinity.");
    bullet("ceil() → rounds toward positive infinity.");
    bullet("round() → rounds to the nearest integer.");
    bullet("trunc() → removes the fractional part.");
}

fn summary() {
    section("Quick reference");

    bullet("f32 = 32-bit floating-point.");
    bullet("f64 = 64-bit floating-point.");
    bullet("f64 is the default floating-point type.");
    bullet("Floating-point values can represent fractional numbers.");
    bullet("Floating-point arithmetic can contain rounding error.");

    println!();

    remember("Use floating-point types when fractional values are required.");
}
