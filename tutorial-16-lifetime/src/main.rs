fn main() {
    /* NOTE:
     * The following code will not compile because the reference `r` is being assigned to a variable
     * that goes out of scope before the reference is used. In this case, r is a dangling reference.
     */

    // let r;

    // {
    //     let x = 5;
    //     r = &x;
    // }

    // println!("r: {}", r);

    // Example #2 (Generic Lifetimes)
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1.as_str(), string2.as_str());
    println!("The longest string is: {}", result);
}

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime

// The example here means that x,y and the return value will have the same lifetime.
// The function signature is saying that for some lifetime 'a, the function takes two parameters
// that are references to strings with that lifetime.
//
// Using a lifetime annotation in a function signature helps the Rust compiler (borrow checker) understand the relationship between the lifetimes of the parameters and return values. This information is crucial for the compiler to ensure that references are always valid (identify dangling references).
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
