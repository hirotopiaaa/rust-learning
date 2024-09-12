fn main() {
    println!("Hello, world!");
    test();
    let result = add_numbers(5, 10);

    println!("The result is: {}", result);

    // assigning the expression to the value of the variable number
    let number = {
        let x = 3;
        x + 1
        // equivalent to
        // return x + 1;
    };
    println!("The number is: {}", number);
}

fn test() {
    println!("Test has been called");
}

fn add_numbers(x: i32, y: i32) -> i32 {
    println!("Adding {} and {}", x, y);
    return x + y;
}
