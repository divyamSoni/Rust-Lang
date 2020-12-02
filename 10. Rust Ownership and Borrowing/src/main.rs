#[allow(unused_variables)]

fn main() {
    let var_a = String::from("Hello");
    let var_b = var_a;
    // Error below ->
    //println!("{}", var_a);

    //Stack
    // - fast memory creation and retrieval..speed
    // - memory is automatically recaptured by thr program after variables go out of scope
    // - is the default in Rust... Collection cannot be stack based
    // String are also collection of u8's, so can be modified hence cannot be on stack!

    let stack_i8: i8 = 10;          // will take 8 bits, whatever we do with it
    let stack_f32: f32 = 20.;       // all are fixed in size
    let stack_bool: bool = true;    // compiler knows the exact
    let stack_char: char = 'a';     // size during compile time

    if stack_i8 == 3 {
        // LIFO
        let inside_scope = 9;
        println!("inside scope:     {}", inside_scope);
    }

    // Heap
    // - flexibility
    // - Memory that can grow in size (Vector, HashMaps, Strings..etc)
    // - Runtime performance cost (speed)
    // - Memory that can live beyond the scope that created it
    // - Memory is automatically recaptured when last owner goes out of scope

    let heap_vector: Vec<i8> = Vec::new(); // vec[5,2]
    let heap_string: String = String::from("Hello");
    let heap_i8: Box<i8> = Box::new(30); // heap_is is the owner of its memory

    let stack_i8_2 = stack_i8; // New mamory is allocated
    println!("stack_i8:     {}", stack_i8); // because heap memory allocation is costly
    println!("stack_i8_2:   {}", stack_i8_2); // rust creates new stack memory!

    /*
    let heap_i8_2  = heap_i8;       // Allocated memory remains intact, ownership is moved to heap_i8_2
    //println!("{}", heap_i8);      // Gives an error
    println!("{}", heap_i8_2);
    */

    let heap_i8_2 = &heap_i8; // 1. Borrow the ownership
    let heap_i8_2 = heap_i8.clone(); // 2. Clone creates a copy if the memory

    println!("heap_i8:  {}", heap_i8); // Gives an error
    println!("heap_i8_2:    {}", heap_i8_2);

    let stack_f64: f64 = 1.;
    let heap_f64: Box<f64> = Box::new(2.);

    stack_procedure(stack_f64);
    println!("In main stack     {}", stack_f64);

    heap_procedure(&heap_f64);
    println!("In main heap      {}", heap_f64);

    let some_string: String = String::from("Hello");    // Strings are always on the heap
    let some_str: &str = "Partner"; // &str is a pointer.. to either stack or heap

    some_procedure(&some_string, some_str);
    println!("{}, {}", some_string, some_str);

    let var_a1 = String::from("Hello");     // referencing = only allowed to read it
    let var_b1 = &var_a1;                   // referencing var_a1, (no change in ownership)
    let var_c1 = &var_a1;                   // referencing var_a1, (no change in ownership)

    println!("{}, {}, {}", var_a1, var_b1, var_c1);
}

fn stack_procedure(mut param: f64) {
    param += 9.;
    println!("In stack_procedure with param     {}", param);
}

fn heap_procedure(param: &Box<f64>) {
    println!("In heap_procedure with param     {}", param);
}

fn some_procedure(param_a: &String, param_b: &str){
    println!("{}, {}", param_a, param_b);
}