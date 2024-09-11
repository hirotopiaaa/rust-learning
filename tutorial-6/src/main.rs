use std::io;

fn main() {
    // "overflow" means that the value is too large to be stored in the data type
    let x: u8 = 127; // 0 to 255
    let y: i8 = 10; // -128 to 127

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    // let z = x + y; // This will cause an overflow
    // println!("The value of z is: {}", z);

    // explicit type casting (conversion)
    let x = 127_000i64; // 0 to 255
                        // let x = 127_000 as i64; // 0 to 255
    let y = 10_i32; // -128 to 127

    let z = x / (y as i64);
    println!("The value of z is: {}", z);

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let int_input: i64 = input.trim().parse().unwrap();

    println!("{}", input);
}
