fn main() {
    println!("Hello, world!");

    another_function();

    function_with_params(5, 6);

    let z = five();
    println!("The value of z is: {}", z);

    let a = plus_one(4);
    println!("The value of a is: {}", a);
}

fn another_function() {
    println!("Another function.");
}

// The type of each param must be declared.
fn function_with_params(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// Functions that return a value to the code that calls them must decalare the
// type of the value returned.
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
