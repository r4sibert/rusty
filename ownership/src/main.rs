// The Rust Language book: Ownership and Functions
fn main() {
    let s = String::from("Hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // and so is no longer valid here.

    let x = 5; // x comes into scope

    makes_copy(x); // Because i32 implements the Copy trait,
                   // x does NOT move into the function,
                   // so it is ok to use x afterward. 

    println!("{x} is still in scope even after the function call");


    // s3 goes out of scope and is dropped. 
    // s2 was moved so nothing happens.
    // s1 goes out of scope and is dropped.
    let s1 = gives_ownership(); // gives_ownership moves it's return 
                                // value into s1

    let s2 = String::from("hello"); // s2 comes into scope

    let s3 = takes_and_gives_back(s2); // s2 is moved into 
                                       // takes_and_gives_back(), which also
                                       // moves its return value into s3
} // 


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // some_string goes out of scope and 'drop' is called. Memory is freed. 


fn makes_copy(some_integer: i32) {
    println!("Printing variable x ({some_integer}) inside a function.");
} // some_integer goes out of scope. Nothing happens. 


fn gives_ownership() -> String { // gives_ownership will move its
                                 // return value into the function
                                 // that calls it. 

    let some_string = String::from("yours"); // some_string comes into scope

    some_string // some_string is returned and 
                // moves out to the calling function
}


fn takes_and_gives_back(a_string: String) -> String {
    a_string // return a_string 
}
