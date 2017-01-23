fn main() {
    let mut s = String::from("Hello"); // This type of String is allocated on the heap so can be mutated
    s.push_str(", world!");
    println!("{}", s); // This will print "Hello, world!"

    // To avoid a double-free error, Rust will invalidate the first reference
    // to allocated memory when it is copied. This behaviour is known as a “move”.
    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{}", s2); // If `s2` were replaced with `s1` here, the program would fail to compile because `s1` has been invalidated

    // To deeply clone the heap data of the String and not just the stack data:
    let s3 = String::from("Hello");
    let s4 = s3.clone();
    println!("s3 = {}, s4 = {}", s3, s4); // This would not fail to compile

    // -------------------------------------------------------------------------

    let s5 = String::from("Hello"); // Bring s5 into scope

    take_ownership(s5); // s5's value moves into the function and is no longer valid.
                        // s5 is now out of the scope of this fn

    // println!("s5 = {}", s5); // This would cause a compilation error

    // -------------------------------------------------------------------------

    let s6 = give_ownership(); // give_ownership() moves its return value to s6

    let s7 = String::from("Hello"); // s7 comes into scope

    let s8 = takes_and_returns(s7); // s7 is moved into takes_and_returns.
                                    // takes_and_returns' return value is moved into s8
} // Here, s6 and s8 go out of scope and are dropped. s7 goes out of scope but was moved so nothing happens in the heap.

fn take_ownership(some_string: String) { // some_string comes into scope
    println!("takes_ownership of: {}", some_string);
} // some_string goes out of scope and it's backing memory is freed

fn give_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}

fn takes_and_returns(some_other_string: String) -> String {
    some_other_string // some_other_string is returned and moves out to the calling function
}
