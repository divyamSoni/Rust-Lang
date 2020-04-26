#[allow(unused_variables)]

fn main() {
    let some_bool = true;
    let some_int1 = 30;
    let some_int2 = 50;

    if some_bool == true || (some_int1 > 100 && some_int2 == 200) {
        println!("Hit if branch")
    } else if some_int1 == 10 {
        println!("Hit else if statement");
    } else {
        println!("Hit else branch");
    }

    let var_from_inline = if some_int1 == 9 { 300 } else { 400 };

    match some_bool {
        true => {
            println!("Hit true branch");
        }
        false => {
            println!("Hit false branch");
        }
    }

    // all possibilities are accounted for
    match some_int1 {
        0 => println!("Hit 0 branch"),
        1..=100 => {
            println!("B/W 1 and 100 branch");
            println!("some more stuff");
        }
        _ => println!("Else branch"),
    }

    let var_from_match1 = match some_bool {
        true => 10,
        false => 20,
    };

    let var_from_match2 = match some_int1 {
        0 => 0,
        1 | 2 => 100,
        _ => 200,
    };
}
