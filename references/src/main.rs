// The Rust Language Book, References and Borrowing
// Variable references are denoted by an ampersand. 
// Referenced variables cannot be modified. 
fn main() {
    
    let s1 = String::from("abracadabra");
    let mut s2 = String::from("hello");

    let len = calculate_length(&s1); // The ampersand signals borrowing.

    println!("");
    println!("The length of '{s1}' is {len}");
    println!("");

    // This function mutates the variable s2 using 
    // mutable refences. Can only reference one variable
    // at a time. 
    change(&mut s2);
    println!("Mutable reference: {s2}");
}

fn calculate_length(s: &String) -> usize { // s contains a string reference. 
    s.len() // the variable s cannot be modified. 
}

fn change(some_string: &mut String) { // This function mutates the borrowed reference.
    some_string.push_str(", world");
}
