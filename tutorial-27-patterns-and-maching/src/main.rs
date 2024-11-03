fn main() {
    // --- Pattern ---

    enum Language {
        English,
        Spanish,
        Russian,
        Japanese,
        Chinese,
    }

    let language = Language::Japanese;

    match language {
        Language::English => println!("Hello World!"),
        Language::Spanish => println!("Hola Mundo!"),
        Language::Russian => println!("Привет мир!"),
        Language::Japanese => println!("こんにちは世界!"),
        Language::Chinese => println!("你好，世界！"),
        _ => println!("Unsupported language!"),
    }

    // --- Conditional if let Expressions ---
    let authorization_status: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "34".parse();

    if let Some(status) = authorization_status {
        println!("Authorization status: {}", status);
    } else if is_admin {
        println!("Authorization status: admin");
    } else if let Ok(group_id) = group_id {
        if group_id > 30 {
            println!("Authorization status: privileged");
        } else {
            println!("Authorization status: basic");
        }
    } else {
        println!("Authorization status: guest");
    }

    // --- While let Conditional Loops ---
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    // match on the result of calling stack.pop(), which returns an optional
    // if the optional is Some variant, then print it  out
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // --- For Loops ---
    let v = vec!['a', 'b', 'c'];

    // enumerate() returns an iterator that yields each element of the vector along with its index
    // use (index, value) as a  pattern to destructure the tuple
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // --- Let Statements ---
    let x = 5;

    // let PAT = EXPRESSION;

    // let statement can be used to destructure a tuple
    let (x, y, z) = (1, 2, 3);
    // ignore the third value
    // let (x, y, _) = (1, 2, 3);

    // --- Function Parameters ---
    let point = (3, 5);
    print_coordinates(&point);

    // --- Irrefutable and Refutable Patterns ---
    // Irrefutable patterns are patterns that will match for any value
    // Refutable patterns are patterns that can fail to match for some values

    // Irrefutable pattern
    let x = 5;

    // Refutable pattern
    let x: Option<&str> = None;
    if let Some(x) = x {
        // the following line will not be executed because x is None
        println!("Some value");
    }

    // Can only accept irrefutable patterns
    // function parameters, let statements, and for loops

    // --- Practical Examples ---
    let x = 1;

    match x {
        1 => println!("One"),
        2 => println!("Two"),
        3 => println!("Three"),
        _ => println!("Anything"),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y),
        _ => println!("Default case, x = {:?}", x),
    }

    // matching multiple patterns
    let x = 3;

    match x {
        1 | 2 => println!("One or Two"),
        3 => println!("Three"),
        _ => println!("Default case, x = {:?}", x),
    }

    // matching range of values
    let x = 5;

    match x {
        1..=5 => println!("One through Five"),
        _ => println!("Something else"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("Early ASCII letter"),
        'k'..='z' => println!("Late ASCII letter"),
        _ => println!("Something else"),
    }

    // destructuring structs
    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // destructuring enums
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.")
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {} and in the y direction {}", x, y)
        }
        Message::Write(text) => println!("Text message: {}", text),
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {}, green {}, and blue {}", r, g, b)
        }
    }

    // ignore values in a pattern
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("Found a string");
    }

    println!("{:?}", s);

    // ignore some parts of a value and destructure the rest
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last)
        }
    }

    // match guards
    // match guards are extra conditions that must be satisfied to run the code in the match arm
    let num = Some(4);
    match num {
        // if the value is Some and the value is less than 5, then print the value
        Some(x) if x < 5 => println!("Less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        // if the value is Some and the value is equal to y, then print the value
        Some(n) if n == y => println!("Matched, n = {}", n),
        _ => println!("Default case, x = {:?}", x),
    }

    // @ Bindings
    enum Message2 {
        Hello { id: i32 },
    }

    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello {
            // bind the value of id to the variable id_variable
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message2::Hello { id: 10..=12 } => println!("Found an id in another range"),
        Message2::Hello { id } => println!("Found some other id: {}", id),
    }
}

fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}
