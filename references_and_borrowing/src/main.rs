fn main() {
    let s1 = String::from("Hello");

    let len = calculate_len(&s1); // [1]

    println!("The length of '{}' is {}.", s1, len);

    // [1]: The `&` denotes a reference. References allow us to refer to a value
    // without taking ownership of it. When `&s1` was passed into the `calculate_len`
    // function, `s1` wasn't moved from the scope.

    // change(&s1); // Uncommenting this and the change line below would cause
                    // a compilation error. Just like variables, references are immutable by default.
    change(&mut s1);
}

// In the following fn, s comes into and goes out of scope as expected. However,
// because it's a reference and no ownership was taken, s1 is not dropped.
// Having references as params is known as borrowing.
fn calculate_len(s: &String) -> usize {
    s.len()
}

// fn change(s: &String) {
fn change(s: &mut String) {
    s.push_str(", world!");
}
