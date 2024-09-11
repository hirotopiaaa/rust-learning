fn main() {
    let mut x: u32 = 4;
    println!("The value of x is: {}", x);
    x = 5;
    println!("The value of x is: {}", x);

    // scope shadowing (interior scope)
    {
        let x = x - 2;
        println!("The value of x is: {}", x);
        let x = 2;
        println!("The value of x is: {}", x);
    }

    let x = x + 1;
    println!("The value of x is: {}", x);

    const SECONDS_IN_MINUTE: u32 = 60;
    println!("{}", SECONDS_IN_MINUTE);
}
