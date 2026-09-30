use crate::utils::{bullet, code, header, info, note, remember, section};

pub fn u_integers() {
    header("UNSIGNED INTEGERS");

    introduction();
    types();
    declarations();
    literals();
    arithmetic();
    overflow();
    conversion();
    summary();
}

fn introduction() {
    section("What is a uN integer?");

    info("uN means an unsigned integer containing N bits.");
    bullet("Unsigned integers cannot represent negative values.");
    bullet("They represent zero and positive values.");

    println!();

    code("u8   = unsigned 8-bit integer");
    code("u16  = unsigned 16-bit integer");
    code("u32  = unsigned 32-bit integer");
    code("u64  = unsigned 64-bit integer");
    code("u128 = unsigned 128-bit integer");
    code("usize = unsigned integer sized for the target platform");

    println!();

    remember("The number after 'u' tells you the integer's bit width.");
}

fn types() {
    section("Available unsigned integer types");

    println!("  Type      Bits      Bytes      Minimum      Maximum");
    println!("  ─────────────────────────────────────────────────────");

    println!(
        "  u8        {:<9} {:<10} {:<12} {}",
        8,
        std::mem::size_of::<u8>(),
        u8::MIN,
        u8::MAX
    );

    println!(
        "  u16       {:<9} {:<10} {:<12} {}",
        16,
        std::mem::size_of::<u16>(),
        u16::MIN,
        u16::MAX
    );

    println!(
        "  u32       {:<9} {:<10} {:<12} {}",
        32,
        std::mem::size_of::<u32>(),
        u32::MIN,
        u32::MAX
    );

    println!(
        "  u64       {:<9} {:<10} {:<12} {}",
        64,
        std::mem::size_of::<u64>(),
        u64::MIN,
        u64::MAX
    );

    println!(
        "  u128      {:<9} {:<10} {:<12} {}",
        128,
        std::mem::size_of::<u128>(),
        u128::MIN,
        u128::MAX
    );

    println!(
        "  usize     target    {:<10} {:<12} {}",
        std::mem::size_of::<usize>(),
        usize::MIN,
        usize::MAX
    );

    println!();

    info("Unsigned integers use all bits for zero and positive values.");
    info("Because there is no negative range, they can represent larger positive values");
    info("than the corresponding signed integer type with the same bit width.");

    remember("u8 ranges from 0 to 255.");
}

fn declarations() {
    section("Declaring unsigned integers");

    let age: u8 = 22;
    let quantity: u32 = 500;
    let large_number: u64 = 1_000_000;

    code("let age: u8 = 22;");
    code("let quantity: u32 = 500;");
    code("let large_number: u64 = 1_000_000;");

    println!();

    info(&format!("age = {age}"));
    info(&format!("quantity = {quantity}"));
    info(&format!("large_number = {large_number}"));

    println!();

    note("A negative value cannot be assigned to an unsigned integer.");
}

fn literals() {
    section("Unsigned integer literals");

    let decimal: u32 = 42;
    let hexadecimal: u32 = 0x2A;
    let octal: u32 = 0o52;
    let binary: u32 = 0b101010;

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

    info("Integer literals can also use an explicit suffix.");

    code("42u8");
    code("42u32");
    code("42u64");
}

fn arithmetic() {
    section("Arithmetic");

    let a: u32 = 20;
    let b: u32 = 6;

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

    info("Unsigned arithmetic cannot produce a negative result.");

    note("Subtracting a larger unsigned value from a smaller one can overflow.");
}

fn overflow() {
    section("Unsigned integer overflow");

    let max = u8::MAX;

    println!("u8 can only store values from {} to {}.", u8::MIN, u8::MAX);

    println!();

    println!("Given:");
    println!("  max = {max}");

    println!();

    println!("What happens if we add 1?");

    println!();

    println!("  checked_add(1)    → {:?}", max.checked_add(1));
    println!("  saturating_add(1) → {}", max.saturating_add(1));
    println!("  wrapping_add(1)   → {}", max.wrapping_add(1));

    println!();

    bullet("checked_add → returns None on overflow.");
    bullet("saturating_add → stops at the maximum value.");
    bullet("wrapping_add → wraps around to zero.");

    remember("Unsigned overflow wraps from MAX back toward MIN when wrapping arithmetic is used.");
}

fn conversion() {
    section("Integer conversion");

    let small: u8 = 10;
    let larger: u32 = small as u32;

    code("let small: u8 = 10;");
    code("let larger: u32 = small as u32;");

    println!();

    info(&format!("small  = {small}"));
    info(&format!("larger = {larger}"));

    println!();

    info("Signed and unsigned integers are different types.");
    info("Rust does not silently mix them in arithmetic.");

    code("let value: u32 = small as u32;");
}

fn summary() {
    section("Quick reference");

    bullet("uN = unsigned integer with N bits.");
    bullet("Unsigned integers cannot represent negative values.");
    bullet("u8 ranges from 0 to 255.");
    bullet("Each unsigned type has a fixed range.");
    bullet("MIN and MAX expose the boundaries.");
    bullet("Unsigned arithmetic can overflow.");
    bullet("Different integer types require explicit conversion.");

    println!();

    remember("Use unsigned integers when negative values are not part of the domain.");
}
