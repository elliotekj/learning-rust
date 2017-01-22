fn main() {
    // loop {
    //     println!("This is an infinite loop!");
    // }

    let mut num = 3;

    while num != 0 {
        println!("{}", num);
        num = num - 1;
    }

    println!("GO!");

    // The above approach, whilst valid, is slow because the compiler adds
    // runtime code to perform the conditional check. A more efficient way is to
    // use a `for` loop to execute some code for each item in a collection.

    let num_arr = [3, 2, 1];

    for item in num_arr.iter() {
        println!("{}", item);
    }

    println!("GO!");

    // An even nicer way to write the above would be to use a range.

    for number in (1..4).rev() {
        println!("{}", number);
    }

    println!("GO!");
}
