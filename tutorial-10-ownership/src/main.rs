fn main() {
    let x: i32 = 5;
    let y: i32 = x; // Copy

    let s1: String = String::from("hello");
    // let s2: String = s1; // Move (not shallow copy)
    let s2: String = s1.clone(); // Clone (deep copy)
    println!("{}, world", s1);
    println!("{}, world", s2);

    let s1: String = gives_ownership();
    let s2: String = String::from("hello");
    let s3: String = takes_and_gives_back(s2);
    println!("s1 = {}, s3 = {}", s1, s3);

    let s4: String = String::from("hello");
    let len = calculate_length(&s4);
    println!("The length of '{}' is {}", s4, len);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("{:?}", slice);
}

fn gives_ownership() -> String {
    let some_string: String = String::from("hello");
    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    some_string
}

// example of borrowing (passing a reference to a function not taking ownership)
// a reference is not mutable by default
fn calculate_length(s: &String) -> usize {
    let length: usize = s.len();
    length
}
