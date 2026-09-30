use std::{io, ops::ControlFlow::Continue};

fn main() {
// Signed Integers
fn i_integers(){
    println!("\x1b[1;34m!-----------------------------------------------\x1b[0m");
    println!("\x1b[1;34mSIGNED INTEGERS\x1b[0m");
        let ai: i8 = 1;
        let bi: i32 = 2;
        let ci: i64 = 3;
        let di: i128 = 4;
    println!("Signed Integers:{}, {}, {}, {}", ai, bi, ci, di);
    println!("\x1b[1;34m!-----------------------------------------------\x1b[0m");
}

// Un Signed Integers
fn u_integers(){
    println!("\x1b[1;34m!-----------------------------------------------\x1b[0m");
    println!("\x1b[1;34mUNSIGNED INTEGERS\x1b[0m");
        let au: u8 = 1;
        let bu: u32 = 2;
        let cu: u64 = 3;
        let du: u128 = 4;
    println!("Un Signed Integers:{}, {}, {}, {}", au, bu, cu, du);
    println!("\x1b[1;34m!-----------------------------------------------\x1b[0m");
}

// Floating Numbers
fn floats(){
    println!("\x1b[1;34m!-----------------------------------------------\x1b[0m");
        println!("\x1b[1;34mFLOAT\x1b[0m");
        let af: f32 = 1.1;
        let bf: f64 = 2.2;
        println!("Floating Numbers:{}, {}", af, bf);
    println!("\x1b[1;34m!-----------------------------------------------\x1b[0m");
    println!("\x1b[1;34m!-----------------------------------------------\x1b[0m");
}

// Booleans
fn booleans(){
    println!("\x1b[1;34m!-----------------------------------------------\x1b[0m");
    println!("\x1b[1;34mBOOLEANS\x1b[0m");
        let t: bool = true;
        let f: bool = false;
        println!("Booleans:{}, {}", t, f);
    println!("\x1b[1;34m!-----------------------------------------------\x1b[0m");
}

    // arrays
fn arrays(){ 
    println!("\x1b[1;34m!-----------------------------------------------\x1b[0m");
    println!("\x1b[1;34mARRAYS\x1b[0m");
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        let fruits: [&str; 3] = ["apple", "banana", "orange"];

        println!("an array:{:?}", arr);
        println!("array element{:?}", fruits[2]);

        //aother way of array initialization
        let a = [3; 5];
        println!("short way: {:?}", a);
        //same as
        let a = [3, 3, 3, 3, 3];
        println!("Long way: {:?}", a);
    println!("\x1b[1;34m!-----------------------------------------------\x1b[0m");
}

    //  tuples
fn tupples(){
    println!("\x1b[1;34m!-----------------------------------------------\x1b[0m");
    println!("\x1b[1;34mTUPPLES\x1b[0m");
        let human: (&str, i32, bool) = ("alice", 22, true);
        
        println!("tuples{:?}", human);
        println!("Accessing  tupple elements");
        println!(
            "Hello {}, your age is {} and you are {}",
            human.0,
            human.1,
            if human.2 { "Married" } else { "Single" }
        );
        
        println!("destructure tupple");
        // destructure a tupple
        let (name, age, married) = human;
        println!(
            "Hello {name}, your age is {age} and you are {}",
            if married { "Married" } else { "Single" }
        );
        
    println!("\x1b[1;34m!-----------------------------------------------\x1b[0m");
}

    // slices
fn slices(){
    println!("\x1b[1;34m!-----------------------------------------------\x1b[0m");
    println!("\x1b[1;34mSLICES\x1b[0m");
        let number_slice: &[i32] = &[1, 2, 3, 4, 5];
        println!("number_slice{:?}", number_slice);

        let animal_slice: &[&str] = &["lion", "elephant", "crocodail"];
        println!("animal_slice{:?}", animal_slice);

        let book_slice: &[&String] = &[
            &"Dexter".to_string(),
            &"House".to_string(),
            &"Person Of Intrest".to_string(),
        ];
        println!("book_slice{:?}", book_slice);
    println!("\x1b[1;34m!-----------------------------------------------\x1b[0m");
}
    // String vs String Slices (&str)
    // String (growable, mutable, owned string type)
fn string_vs_string_slice(){
    println!("\x1b[1;34m!-----------------------------------------------\x1b[0m");
    println!("\x1b[1;34mSTRING VS STRING SLICE\x1b[0m");
        let mut stone_cold: String = String::from("Hell, ");
        println!("String sayes Hello From {}", stone_cold);
        stone_cold.push_str("Yeah!");
        println!("String sayes Hello From {}", stone_cold);

        // String Slice (&str)
        // Immutable
        let string: String = String::from("Hello From the String Borrowed ");
        let slice: &str = &string;
        let sliceLimit: &str = &string[0..10];

        println!("Slice Value: {}", slice);
        println!("Slice Limit Value: {}", sliceLimit);
    println!("\x1b[1;34m!-----------------------------------------------\x1b[0m");
}

    //Shadowing and scoping
fn shadowing(){
    println!("\x1b[1;34m!-----------------------------------------------\x1b[0m");
    println!("\x1b[1;34mSHASOWING\x1b[0m");
        let x = 5;

        let x = x + 1;

        {
            let x = x * 2; //x = 12 here
            println!("The value of x in the inner scope is: {x}");
        }

    println!("The value of x is: {x}"); //x = 6 here
    println!("\x1b[1;34m!-----------------------------------------------\x1b[0m");
}
const SUBJECTS_COVERED:[&str;8] = ["Signed Integers", "UnSigned Integers", "Floats", "Booleans", "Arrays", "Tupples", "String vs String Slice", "Variable Shadowing"];

println!("Wellcome to Rust guide made by @Zaki");
println!("Please pick a sucject");
loop {
    
     println!("\nSubjects:");

    for (index, subject) in SUBJECTS_COVERED.iter().enumerate() {
        println!("[{}] {}", index + 1, subject);
    }
    
    let mut subject = String::new();
    
    io::stdin()
    .read_line(&mut subject)
    .expect("Fild to read line");

match subject.trim() {
    "1" => i_integers(),
    "2" => u_integers(),
    "3" => floats(),
    "4" => booleans(),
    "5" =>arrays(),
    "6" =>tupples(),
    "7" =>string_vs_string_slice(),
    "8" =>shadowing(),
    _   => println!("Subject not found on the list {}",subject.trim()),
    }
    println!("Explore more y/n");
    let mut con = String::new();
    io::stdin().read_line(&mut con).expect("Faild to read line!");
    match con.trim() {
        "n" => break,
        "y" => continue,
        _   => continue
    }

}




}
