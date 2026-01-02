// The Rust Language Book - Loops and flow control.

fn while_for() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("In the while-loop, the value of array at index {} is: {}", index, a[index]);

        index += 1;
    }

    println!("");
    
    // For loops are safer when iterating over an array - no off by one bugs. 
    for element in a {
        println!("In the for-loop, the value of array is: {}", element);
    }

    // A for loop using a range:
    println!("");
    println!("Now, a countdown:");
    for number in (1..5).rev() { // Pretty sure the .rev() means to reverse the range.
        println!("{number}!");
    }
    println!("Liftoffffffffffffffff");
}

fn main() {
    let mut counter = 0;
    // the single quote in front of 'counting_up is a 'loop label'.
    // This allows the break command in the innermost loop to
    // break the outer 'counting_up loop. Neato.
    'counting_up: loop {
        println!("count = {counter}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if counter == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        counter += 1;
    }

    println!("End count = {counter}");
    println!("");

    while_for()

}
