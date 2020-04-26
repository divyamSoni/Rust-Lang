#[allow(unused_variables)]
#[allow(unused_assignments)]

fn main(){
    // Boolean
    // use 'let' to create a new variable
    let some_data: bool = true;
    // Mutability ?
    // Mutable      - Can be changed
    // Immutable    - Can't be changed
    // Rust by default assume things are immutable
    // some_data = false; // will give a compile error

    // to make a variable mutable 
    let mut some_other_data: bool = true;
    some_other_data = false;

    // Integer
    let some_integer: i8 = 10;  // i = integer, 8 = 8 bits of memory
                                // enough memory to store 256 = -128 to +127
    println!("Min is {}", std::i8::MIN);
    println!("Max is {}", std::i8::MAX);

    let test_int = some_integer + 120;  // 138, which is greater than the max 128
    println!("{}", test_int);           // This will PANIC, i.e. program crashes
    // debug mode and release mode treat things differently-

    // cargo run --release will wrap around and make
    // test_int = -126 instead of 138 which is wrong!!

    let unsigned_integer: u8 = 10;      // from 0 to 255
    println!("Min is {}", std::i128::MIN);
    println!("Max is {}", std::i128::MAX);

    // if type of integer is not specified explicitely,
    // it will assume i32 by default

    let some_isize: isize = 10;         // dependes on computer - 32 bit or 64 bit
    let some_usize: usize = 10;         // same here

    // Float
    let some_float: f32 = 10.;          // Don't forget the decimal point
    // system will assume f64 by default, if not specified
    
    // Character
    let some_char: char = 'a';          // More than just ascii
    // char surrounded by single quotes ''
    // 4 bits
    // can handle languages, even emojis :)
}