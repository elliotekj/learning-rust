fn main() {
    let number = 4;

    // Rust does not try to convert non-boolean types to a boolean like JS or
    // Ruby. If the following line was `if number {` the the program would fail
    // to compile.
    if number < 5 {
        println!("Condition was true.");
    } else {
        println!("Condition was false.");
    }

    if number != 0 {
        println!("`number` was something other than 0.");
    }

    if number % 4 == 0 {
        println!("`number` is divisible by 4.");
    } else if number % 3 == 0 {
        println!("`number` is divisible by 3.");
    } else {
        println!("`number` is not divisible by 4 or 3.");
    }

    // `if` is an expression so we can use it on the right side of a let statement.
    // Note that each arm must evaluate to the same type. If the `else` arm returned
    // `"string"` the program would fail to compile.
    let condition = true;
    let x = if condition {
        4
    } else {
        5
    };
    println!("The value of x is: {}", x);
}
