// Exercise 2 from The Rust Language book
// 12/2025

fn main() {
    let x = 5;
    let x = x + 1; // variable shadowing.

    // Tuples are immutable
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (a, b, _c) = tup;

    println!("The value of a is {a}");
    println!("The value of b is {b}");

    let d = tup.2;
    println!("The value of c is {d} (from tup.2)");

    // Arrays are also immutable, but allocated on the stack rather than the heap. 
    let e = [1, 2, 3, 4, 5, 6];
    let first = e[0];
    println!("The first element of the array is {first}");


    {
        let x = x * 2; // another variable shadow in a different scope.
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}"); // inner scope dropped. 
                                        //
                                        //
    another_func(5);
}

// Shadowing required the use of 'let'. Unlike 'mut', shadowing 
// effectively rewrites the variable, including as different types, e.g.:
//
// let spaces = "   "; (a string)
// let spaces = spaces.len(); (a number)
//
// whereas this will throw an error:
// 
// let mut spaces = "    ";
// let spaces = spaces.len();  

fn another_func(x: i32) {
    
    println!("Another function but takes an argument. Here: {x}");
}
