struct MyStruct<T, U>
where T: std::fmt::Debug,
      U: std::fmt::Debug{
    my_t: T,
    my_u: U,
}

impl<T, U> MyStruct<T, U>{
    fn log_something(&self){
        println!("{:?}{:?}", self.my_t, self.my_u);
    }
}

enum SomeEnum<T>{
    OptionA(T),
    OptionB(T),
    OptionC,
}

struct Point<T, U>{
    x: T,
    y: U,
}

fn main(){
    let a = Point{x:100, y:-1};
    println!{"x = {} y = {}", a.x, a.y};

    let b = Point{x:10.1, y:-2.5};
    println!{"x = {} y = {}", b.x, b.y};

    let some_data = SomeEnum::OptionA(34.2);

    match some_data{
        SomeEnum::OptionA(a) => println!("OptionA contains {}", a),
        SomeEnum::OptionB(b) => println!("OptionB contains {}", b),
        SomeEnum::OptionC => println!("OptionC contains :("),
    }

    let some_data2 = SomeEnum::OptionB('b');
    let some_data3 = SomeEnum::OptionA(vec![1,2,3]);

    let c = my_func1(4.4, 5.5);
    println!("c has {}", c);

    let test = MyStruct{
        my_t: 5.6,
        my_u: vec![1, 2, 3],
    };
}

fn my_func1<T: std::ops::Add<Output=T> + std::ops::Sub<Outputs=T> + std::fmt:Debug>(input_a: T, input_b: T) -> T{
    println!("input_a has {:?}", input_a);
    input_a + input_b
}

#[allow(dead_code)]
fn my_func2<T>(input_a: T, input_b: T, input_e: E) -> T
where T: std::ops::Add<Output=T> + std::ops::Sub<Outputs=T> + std::fmt:Debug,
      E: std::fmt:Debug{
    println!("input_a has {:?}", input_a);
    input_a + input_b
}
