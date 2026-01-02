// The Rust Language book: Variable Scope

fn stringss() {
    // String literals are immutable by design. 
    // They are allocated to the stack at compilation time. 
    // To create a mutable string, assign it to a 'String' type. 
    // String types are allocated to the heap. 
    println!("");
    println!("We are in the string function now.");
    println!("");

    let mut s = String::from("hello"); // only available in the function scope

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{s}"); 

    println!("");
    s = String::from("Original string out of scope");
    println!("{s}"); 


    // Cloning variables creates two separate copies on the heap. 
    // Setting two variables equal to one another, e.g., s2 = s1
    // the program discards the original reference. Only one copy
    // remains. 
    println!("");
    println!("variable clones:");
    let s1 = String::from("stringthing");
    let s2 = s1.clone();
    println!("Variable s1 = {s1} and variable s2 = {s2}");
    
    // This creates two copies on the stack without calling clone
    // Integers are of a known size and so are inexpensive to copy.
    let x = 5;
    let y = x;

    // The s variable is dropped from memory at the closing
    // curly brace because it is going out of scope. 
}


fn main() {

    // Outside scope
    let s = "hello";
    println!("{s}");

    {
        // Inside scope
        let s = "goodbye";
        println!("{s}");
        // Inside scope dropped. 
    }

    // Back in the outside scope again. 
    println!("{s}");

    // Yet another use of scope
    stringss();

}
