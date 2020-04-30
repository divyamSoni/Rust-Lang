// Rust handles polymorphism a little differently 
// than other languages, using "Traits"

pub trait SomeTrait{
    fn is_valid(&self) -> bool;
    // fn get_the_better_one(&self, some_other_var: &self) -> Self;

}
// implement "Trait that is being implemented" for "Struct implemeted for"
impl SomeTrait for RandomInfo{
    // compile error if is_valid is not implemented
    // traits are public no need of pub keyword
    fn is_valid(&self) -> bool{
        self.some_bool
    }
}
#[derive(Debug, Copy, Clone)]
pub struct RandomInfo {
    pub call_count: i64,
    pub some_bool: bool,
    pub some_int: i64,
}

// why defining methods/functions outside the structure?
// 1. easy to augument someone elses struct
// 2. in opps class is inhereted and functions are added
//      in rust add any function, anywhere you like

// Implementation
impl RandomInfo {
    // here 'Self' keyword is used
    // that indicate it should be a function available to
    // the type itself access by the two colons
    pub fn new(param_a: bool) -> Self { // Self is type
        Self {
            call_count: 0,
            some_bool: !param_a,
            some_int: 8,
        }
    }
    // self represents actual data
    
    // if reading or writing to a variable using lower
    // case self keyword that will use a dot operator
}

// A struct does'nt need to have any fields
// and is called a uni struct
#[allow(dead_code)]
struct LookNoFields{}

#[allow(dead_code)]
struct Pair<T> {x:T, y:T,}

#[allow(dead_code)]
struct Color(u8, u8, u8);
