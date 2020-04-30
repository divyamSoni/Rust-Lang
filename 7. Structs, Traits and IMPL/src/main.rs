mod random_info;
use random_info::*;

#[allow(dead_code)]
#[derive(Debug)]
// Rust doesn't support inheritance
// Rust has composition over inheritance
struct MyData {
    // Data elements
    some_bool: bool,
    some_float: f64,
    some_int: i32,
    random: RandomInfo,
}
#[allow(dead_code)]
impl RandomInfo{
    pub fn is_larger(&self, compare_to: i64) -> bool{
        self.some_int > compare_to
    }
}
impl SomeTrait for MyData{
    fn is_valid(&self) -> bool{
        true
    }
}

fn print_if_is_valid(check_me: &dyn SomeTrait){
    if check_me.is_valid(){
        println!("Yay!");
    }
}

impl Default for MyData{
    fn default() -> Self{
        Self{
            some_bool: true,
            some_float: 18.9,
            some_int: 80,
            random: RandomInfo::new(true),
        }
    }
}

#[allow(unused_variables)]
fn main(){
    //Rust will assume everything private unless specified
    let random_info_var = RandomInfo{
        call_count: 0,
        some_bool: true,
        some_int: 10,
    };

    let my_var1 = MyData{
        some_bool: true,
        some_float: 10.3,
        some_int: 80,
        random: RandomInfo::new(true),
    };

    let my_var1 = MyData::default();

    // Variables are immutable by default!!!!
    println!("{:?}", my_var1);

    print_if_is_valid(&random_info_var);
    print_if_is_valid(&my_var1);
}
