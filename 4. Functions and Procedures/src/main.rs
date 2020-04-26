#[allow(unused_variables)]

fn main(){
    let returned_data = some_function(2.2, 50);
    println!("returned_data is {}", returned_data);

    some_procedure(2.3, 1);

    some_str_procedure("test");

    let string_slice_var: &str = "Hello";
    some_str_procedure(string_slice_var);

    let string_var = String::from("I'm a REAL String :)");
    some_str_procedure(&string_var);
    some_str_procedure(&string_var); //doesn't give an error

    some_string_procedure(&string_var);
    // some_string_procedure(string_var); // gives an error
    some_string_procedure(&string_var);
    
}

fn some_string_procedure(param: &String){
    println!("I'm in some_string_procedure with param {}", param);
}

fn some_str_procedure(param: &str){
    println!("I'm in some_str_procedure with param {}", param);

}

fn some_procedure(param_a: f32, param_b: i8){
    println!("I'm in some_procedure with a {} b{ }", param_a, param_b);
}

fn some_function(param_a: f32, param_b: i128) -> f32 {
    println!("I'm in some function!");

    // 10.1            // No semicolon means this is what's returned by the function
    // 10 as f32       // casting
    // 10_f32
    // 10.1 * param_a

    if param_a < 100.{
        let return_var = 10.1 * param_a + param_b as f32;
        return_var      // No semicolon, means return
    }
    else{
        -1.
    }
}