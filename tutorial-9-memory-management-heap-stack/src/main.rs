// The difference between stack and heap in Rust is that
// Stack can only store variable with fixed size, while heap can store variable with dynamic size.
// e.g. stack: let x: i32 = 5; heap: let x = Box::new(5);

fn main() {
    let x = 2;
    // compared to "hello" which is a string literal, this is a string object
    // string objects are stored in the heap
    // string literals are stored in the stack (fixed size)
    // String::from() creates a dynically sized string object,.which we can add characters to,
    // remove characters from, and so on because it is stored in the heap
    // much more time consuming than string literals because it needs to look up the data from the
    // heap; while string literals are stored in the stack, which is faster to access (no need lookup up)
    let string = String::from("hello");
}
