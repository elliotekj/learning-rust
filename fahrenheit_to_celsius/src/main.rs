use std::io;

fn main() {
    println!("What temperature would you like to convert to celsius?");

    let mut input_temp = String::new();

    io::stdin().read_line(&mut input_temp)
        .expect("Failed to read line.");

    let input_temp = input_temp.trim().parse::<f64>().unwrap();

    let converted_temp: f64 = (input_temp - 32.0) * 5.0/9.0;

    println!("{} degrees fahrenheit is {} degrees celsius.", input_temp, converted_temp);
}
