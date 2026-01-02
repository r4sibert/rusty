fn main() {

    print_labeled_measurement(5, 'h');
    another_function(6);
    let test = give_value(2);
    println!("The value of test is {test}");
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn give_value(x: i32) -> i32 { // the -> defines the type of the 
                               // return value of the function.
    let y = x * 2;
    y // the return value does not get a semicolon.
}
