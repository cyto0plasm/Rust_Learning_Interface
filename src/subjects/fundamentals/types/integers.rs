use crate::utils::{bullet, code, header, info, note, remember, section};

pub fn i_integers() {
    header("SIGNED INTEGERS");

    introduction();
    types();
    declarations();
    type_inference();
    literals();
    arithmetic();
    methods();
    min_max();
    overflow();
    conversion();
    summary();
}

fn introduction() {
    section("What is an iN integer?");

    info("iN means a signed integer containing N bits.");
    bullet("Signed integers can store negative values.");
    bullet("They can store zero.");
    bullet("They can store positive values.");

    println!();

    code("i8   = signed 8-bit integer");
    code("i16  = signed 16-bit integer");
    code("i32  = signed 32-bit integer");
    code("i64  = signed 64-bit integer");
    code("i128 = signed 128-bit integer");
    code("isize = signed integer sized for the target platform");

    println!();

    remember("The number after 'i' tells you the integer's bit width.");
}

fn types() {
    section("Available signed integer types");

    println!("  Type      Bits      Bytes      Minimum                 Maximum");
    println!("  ───────────────────────────────────────────────────────────────");

    println!(
        "  i8        {:<9} {:<10} {:<23} {}",
        8,
        std::mem::size_of::<i8>(),
        i8::MIN,
        i8::MAX
    );

    println!(
        "  i16       {:<9} {:<10} {:<23} {}",
        16,
        std::mem::size_of::<i16>(),
        i16::MIN,
        i16::MAX
    );

    println!(
        "  i32       {:<9} {:<10} {:<23} {}",
        32,
        std::mem::size_of::<i32>(),
        i32::MIN,
        i32::MAX
    );

    println!(
        "  i64       {:<9} {:<10} {:<23} {}",
        64,
        std::mem::size_of::<i64>(),
        i64::MIN,
        i64::MAX
    );

    println!(
        "  i128      {:<9} {:<10} {:<23} {}",
        128,
        std::mem::size_of::<i128>(),
        i128::MIN,
        i128::MAX
    );

    println!(
        "  isize     target    {:<10} {:<23} {}",
        std::mem::size_of::<isize>(),
        isize::MIN,
        isize::MAX
    );

    println!();

    info("Larger integer types can represent a larger range of values.");
    info("Larger types also require more memory.");

    remember("i32 is Rust's default integer type when no other type is inferred.");
}

fn declarations() {
    section("Declaring signed integers");

    let age: i32 = 22;
    let temperature: i8 = -10;
    let huge_number: i128 = 1_000_000_000_000;

    code("let age: i32 = 22;");
    code("let temperature: i8 = -10;");
    code("let huge_number: i128 = 1_000_000_000_000;");

    println!();

    info(&format!("age = {age}"));
    info(&format!("temperature = {temperature}"));
    info(&format!("huge_number = {huge_number}"));
}

fn type_inference() {
    section("Type inference");

    let number = 42;

    code("let number = 42;");

    println!();

    info(&format!("number = {number}"));

    println!();

    info("Rust can infer the type from how a value is used.");
    info("If there is no information to determine another integer type,");
    info("an integer literal defaults to i32.");

    println!();

    code("let number: i64 = 42;");
    code("let number: i8 = 42;");

    remember("You don't always need to write the type explicitly.");
}

fn literals() {
    section("Integer literals");

    let decimal = 42;
    let hexadecimal = 0x2A;
    let octal = 0o52;
    let binary = 0b101010;

    println!("Different bases can represent the same value:");

    println!();

    code("42        → decimal");
    code("0x2A      → hexadecimal");
    code("0o52      → octal");
    code("0b101010  → binary");

    println!();

    info(&format!("decimal     = {decimal}"));
    info(&format!("hexadecimal = {hexadecimal}"));
    info(&format!("octal       = {octal}"));
    info(&format!("binary      = {binary}"));

    println!();

    info("Underscores can be used to make large numbers easier to read.");

    code("1_000_000");
    code("0b1111_0000");
}

fn arithmetic() {
    section("Arithmetic");

    let a: i32 = 20;
    let b: i32 = -6;

    println!("Given:");
    println!("  a = {a}");
    println!("  b = {b}");

    println!();

    println!("  Addition       : {}", a + b);
    println!("  Subtraction    : {}", a - b);
    println!("  Multiplication : {}", a * b);
    println!("  Division       : {}", a / b);
    println!("  Remainder      : {}", a % b);

    println!();

    info("Integer division produces an integer.");

    code("7 / 2 = 3");

    println!();

    info("The remainder operator (%) gives what remains after division.");

    code("7 % 2 = 1");

    remember("/ gives the quotient; % gives the remainder.");
}

fn methods() {
    section("Useful integer methods");

    let number: i32 = -42;

    println!("Given:");
    println!("  number = {number}");

    println!();

    println!("  abs()    → {}", number.abs());
    println!("  signum() → {}", number.signum());
    println!("  min(10)  → {}", number.min(10));
    println!("  max(10)  → {}", number.max(10));

    println!();

    bullet("abs() returns the absolute value.");
    bullet("signum() returns -1, 0, or 1 depending on the sign.");
    bullet("min() returns the smaller of two values.");
    bullet("max() returns the larger of two values.");
}

fn min_max() {
    section("MIN and MAX");

    println!("Every integer type provides MIN and MAX constants.");

    println!();

    code("i8::MIN");
    code("i8::MAX");
    code("i32::MIN");
    code("i32::MAX");

    println!();

    println!("  i8::MIN  = {}", i8::MIN);
    println!("  i8::MAX  = {}", i8::MAX);

    println!();

    println!("  i32::MIN = {}", i32::MIN);
    println!("  i32::MAX = {}", i32::MAX);

    println!();

    remember("MIN and MAX tell you the boundaries of an integer type.");
}

fn overflow() {
    section("Integer overflow");

    let max = i8::MAX;

    println!("i8 can only store values from {} to {}.", i8::MIN, i8::MAX);

    println!();

    println!("Our value:");
    println!("  max = {max}");

    println!();

    println!("What happens if we add 1?");

    println!();

    println!("  checked_add(1)    → {:?}", max.checked_add(1));
    println!("  saturating_add(1) → {}", max.saturating_add(1));
    println!("  wrapping_add(1)   → {}", max.wrapping_add(1));

    println!();

    bullet("checked_add → returns None when overflow would happen.");
    bullet("saturating_add → stops at the type's MIN/MAX boundary.");
    bullet("wrapping_add → wraps around to the other side.");

    note("Choose the overflow behavior your program actually needs.");

    remember("Integer types have fixed ranges. Values outside the range cause overflow.");
}

fn conversion() {
    section("Integer conversion");

    let small: i8 = 10;
    let large: i32 = small as i32;

    code("let small: i8 = 10;");
    code("let large: i32 = small as i32;");

    println!();

    println!("  small = {small}");
    println!("  large = {large}");

    println!();

    info("Different integer types are not automatically interchangeable.");
    info("Use an explicit conversion when you need another integer type.");

    println!();

    code("let value: i32 = small as i32;");

    note("The 'as' keyword performs a primitive numeric cast.");
}

fn summary() {
    section("Quick reference");

    bullet("iN = signed integer with N bits.");
    bullet("Signed integers can represent negative values.");
    bullet("i32 is the default integer type for integer literals.");
    bullet("Each integer type has a fixed range.");
    bullet("MIN and MAX expose the range boundaries.");
    bullet("Integer division discards the fractional part.");
    bullet("Different integer types require explicit conversion.");
    bullet("Overflow can be handled with checked/saturating/wrapping APIs.");

    println!();

    remember("Think: i8 → small range, i32 → general-purpose, i64/i128 → larger ranges.");
}
