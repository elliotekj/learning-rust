fn main() {
    // Without the `u32` type annotation here, this would fail to compile because the value's type
    // cannot be infered.
    // let guess: u32 = "42".parse().unwrap();

    // Integers default to `i32`
    // let x = 12; // Has type of `i32`

    // Floating point numbers default to `f64`
    // let x = 2.0; // Has a type of `f64`
    // let y: f32 = 3.0; // Has a type of `f32`

    // Booleans
    // let t = true;
    // let f: bool = false; // With explicit type annotation

    // Characters: `char` (represents a Unicode Scalar Value) can represent a lot more than just
    // ASCII → accents, Chinese, Japanese, Korean, emoji, etc.
    // let c = 'c';
    // let e = '🤘';

    // Tuples: A way of grouping a number of values with different types into one compound type.
    // let tup = (100, 6.4, 12);
    // let (x, y, z) = tup; // Known as destructuring through pattern matching
    // let one_hundred = tup.0;
    // println!("The value of y is: {}", y);

    // Arrays: A way of grouping values of the same type into a compound type.
    // Note that in Rust, arrays have a fixed length.
    // let arr = [1, 2, 3, 4, 5];
    // let fourth = arr[3];
    // println!("The value at the fourth index is: {}", fourth);
}
