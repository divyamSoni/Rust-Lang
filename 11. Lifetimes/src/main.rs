#[allow(unused_variables)]

fn main(){
    let some_int_var = 10;
    let some_int_var1 = 20;
    let result_ref = get_int_ref(&some_int_var, &some_int_var1);
    println!("{}", result_ref);
}

// Defining some generic lifetime and my i/p and o/p
// comply with it
fn get_int_ref<'a>(param_1: &'a i32, param_2: &'a i32) -> &'a i32{     // i/p memory and o/p memory
    if param_1 > param_2{
        param_1                                                        // live for the same scope
    }
    else{
        param_2
    }                                            
}

/*  if output has a refrence then the compiler has to know what
    lifetime to assign to the output
fn some_int_ref<'a,'b>(param_1: &'a str, param_2: &'b str, param_3: Vec<64>) -> &str
*/

