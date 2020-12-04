
fn main(){

    // Casting

    let some_i32: i32 = 10;
    let some_f64: f64 = 20.2;
    
    let combined = some_i32 + some_f64 as i32;
    println!("{}", combined);

    // Shadowing
    // different variables with same name 
    // in different scopes
    let var_a: i32 = 10;
    {// any scope
        println!("Inner scope can see the outer scope var_a of {}", var_a);
        let var_a: f32 = 20.213;
        println!("Shadowed it with its own version of f64 {}", var_a);
    }
    println!("Outer scope var_a {}", var_a);

    // Constants
    const MY_CONSTANT: i64 = 100;   // memory not allocated during compile time
    println!("{}", MY_CONSTANT);    // already in the compiled code

    let circle_pi = std::f32::consts::PI;
    println!("{}", circle_pi);

    // Static
    static mut  MY_STATIC_VAR: i32 = 10;

    unsafe{
        MY_STATIC_VAR = 20;

        println!("{}", MY_STATIC_VAR);
    }

}