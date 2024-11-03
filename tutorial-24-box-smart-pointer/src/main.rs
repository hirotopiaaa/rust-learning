use std::cell::RefCell;
use std::rc::Rc;

// Recursive data types with Box<T>
#[derive(Debug)]
enum List {
    // Cons(i32, Box<List>),
    // Cons(i32, Rc<List>),
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

use List::{Cons, Nil};

fn main() {
    // -- Box Smart Pointer --

    // Example 1
    let b = Box::new(5);
    println!("b = {}", b);

    // Example 2
    // let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // -- Deref Trait --

    // Example 1
    let x = 5;
    // let y = &x;
    let y = Box::new(x); // Box points to the value of x on the heap

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = Box::new(String::from("Rust"));
    hello(&m);
    // &Box<String> -> &String -> &str
    // equivalent to hello(&(*m)[..]);

    // -- Reference Counting --

    // Example 1
    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // calling clone() increments the reference count
    // let b = Cons(3, Rc::clone(&a));
    // let c = Cons(4, Rc::clone(&a)); // a is shared between b and c

    // -- Interior Mutability --

    // Example 1
    let a = 5;
    let b = &a;

    let mut c = 10;
    let d = &mut c;
    *d = 20;

    // Example 2
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}
