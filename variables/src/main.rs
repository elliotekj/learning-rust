fn main() {
    // Variables are immutable by default, the following would fail to compile.
    // let x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // This will compile because `x` is mutable.
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // This is known as variable shadowing:
    // let x = 5;
    // let x = x + 1;
    // let x = x * 2;
    // println!("The value of x is: {}", x);

    // This will compile even though they're different types because we re-create `spaces` as a brand new variable:
    // let spaces = "    ";
    // let spaces = spaces.len();
    // println!("The length of spaces is: {}", spaces);

    // This will not compile becuase we can't mutate `x`'s type:
    // let mut spaces = "    ";
    // spaces = spaces.len();
    // println!("The length of spaces is: {}", spaces);
}
