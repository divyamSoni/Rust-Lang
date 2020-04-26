#[allow(unused_variables)]
#[allow(unused_assignments)]

// In other languages
// 1. Hide the complexity
// 2. Makes things like strings easier
// 3. Sacrifices more complex topics

// In Rust
// 1. Makes developer handle the complexity
// 2. Benefits runtime speed, concurrency etc.
// 3. Simple things are harder

fn main() {
    // There are two types of String in Rust
    // 1. String 
    // 2. String slice

    let example_str: &str = "Hello"; // &str represents a grouping of characters
                                     // i.e. immutable for the most part

    let example_string: String = String::from("World"); // Just like other languages
                                                        // Both String and &str are grouping of characters (u8's)
                                                        // differences in how they are stored in the memory
                                                        //                how programmer uses
                                                        // Strings are stored in Heap
                                                        //                       hence Mutable
                                                        // &str is immutable
                                                        // often allocated on the stack, sometimes a heap reference,
                                                        // sometimes embedded in the code
                                                        // Can translate b/w String and str
                                                        // String is great for mutating and holding data for longer
                                                        // str is great for runtime speed

    // Translating b/w the two
    let string_from_str1: String = example_str.to_string();

    let string_from_str2: String = "Some Hardcoded String".to_string();
    // hardcoded strings are called string literals
    // and are string slices that are held in the programms binary
    // or static memory
    let string_from_hardcoded = String::from("Some hardcoded");
    let string_from_str_var = String::from(example_str);

    let str_from_string: &str = &example_string;
    // Compile error
    // let test = "first" + "second";

    let combine_string_literal = ["first", "second"].concat(); // Result will be a String not a string slice
    let combine_with_format_macro = format!("{} {}", "first", "second");

    // Must be String + String slice
    // (concept of borrowing)
    let string_plus_str = example_string + example_str;

    // Blank String that can be mutated/modified
    let mut mut_string = String::new();
    mut_string.push_str(example_str);
    mut_string.push_str("Some hardcoded literal");
    mut_string.push('m');
    // Compile error
    // mut_string.push("m");

    let a = String::from("a");
    let b = String::from("b");
    let combine = a + &b; // Translating each additional string to a string slice
                          // so it can be added
                          // For substrings
    let str_from_substring: &str = &example_str[0..2]; // upto not including
    let str_from_substring: &str = &example_str[0..=2]; // upto including

    let char_by_index = &example_str.chars().nth(1); // nth char may or may not exists

    match char_by_index {
        Some(c) => println!("Found a char {}", c),
        None => {}
    }

    if let Some(c) = example_str.chars().nth(2) {
        println!("Found a char {}", c);
    }
}
