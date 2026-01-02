fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn main() {
    let z = plus_one(14);
    let x = {
        let y = five(); // This block is an arguement. Will not bind to x by itself.
        y + 1 // This block is a statement that evaluates to 6
    };
    println!("The value of x is {x}");
    println!("The value of z is {z}");
}


