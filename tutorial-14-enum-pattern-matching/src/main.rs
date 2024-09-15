enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("Message called");
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let localhost = IpAddrKind::V4(127, 0, 0, 1);

    // Option<T> enum
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // unwrap() returns the value inside the Some variant
    // if the value is None, unwrap() will panic
    // default value can be provided using unwrap_or(), in this case 0
    let sum = x + y.unwrap_or(0);

    // match
    value_in_cents(Coin::Penny);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    let some_value = Some(3);
    match some_value {
        // If the value is 3, print "three"
        Some(3) => println!("three"),
        _ => (),
    }
}

// match
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i8>) -> Option<i8> {
    match x {
        // Some(i) pattern will match any Some value, binding the inner value to i
        Some(i) => Some(i + 1),
        // None pattern will match only the None variant
        // none => None,
        // _ pattern will match any other value
        _ => None,
    }
}
