## Tutorial 25 - Smart Pointers

## Table of Contents

<!-- vim-markdown-toc GFM -->

* [The Book](#the-book)
* [Summary](#summary)
* [Notes](#notes)
    * [Box<T>](#boxt)
    * [Rc<T> (Reference Counted)](#rct-reference-counted)
    * [Arc<T> (Atomic Reference Counted)](#arct-atomic-reference-counted)
    * [RefCell<T>](#refcellt)
    * [Mutex<T> and RwLock<T>](#mutext-and-rwlockt)

<!-- vim-markdown-toc -->

### The Book

Chapter 15: Smart Pointers

### Summary

In Rust, smart pointers are a type of data structure that not only act as pointers (reference types) but also provide additional capabilities, such as automatic memory management and reference counting. Rust has several types of smart pointers that handle ownership, borrowing, and automatic cleanup in unique ways, which helps manage memory safely and efficiently.

- **Box<T>**: A smart pointer that allows you to store data on the heap rather than the stack. It's useful when you need to allocate memory at runtime or when you have a large amount of data that you want to own.
- **Rc<T>**: A reference-counted smart pointer that allows multiple owners to share immutable data. It keeps track of the number of references to a value and automatically cleans up the value when the last reference is dropped.
- **Arc<T>**: An atomic reference-counted smart pointer that allows multiple threads to share immutable data. It's similar to `Rc<T>` but provides thread-safe reference counting using atomic operations.
- **RefCell<T>**: A smart pointer that provides interior mutability, allowing you to mutate data even when it's immutably borrowed. It enforces borrowing rules at runtime rather than compile time.
- **Mutex<T>**: A mutual exclusion smart pointer that allows multiple threads to access mutable data in a synchronized way. It provides a lock that must be acquired before accessing the data and released after the access is complete.
- **RwLock<T>**: A reader-writer lock smart pointer that allows multiple readers or one writer to access mutable data. It enforces exclusive access to the data, allowing multiple readers or one writer at a time.

These smart pointers help manage memory and enforce Rust’s borrowing and ownership rules, providing flexible but safe ways to handle data sharing and concurrency.

### Notes

#### Box<T>

- The simplest smart pointer in Rust, `Box<T>`, allocates data on the heap rather than the stack.
- Useful when you need to transfer large data without copying, or to store types whose size isn’t known at compile time (e.g., recursive data structures).

Example:

```rust
let b = Box::new(5);
println!("b = {}", b);
```

- `Box<T>` has single ownership, meaning only one owner exists at a time, so no reference counting or cloning overhead exists.

#### Rc<T> (Reference Counted)

- `Rc<T>` is a reference-counted smart pointer used for multiple ownership scenarios where a value needs to be shared.
- Useful when different parts of a program need to read from the same data without modifying it.
- Only works in single-threaded environments since it’s not thread-safe.

Example:

```rust
use std::rc::Rc;

let a = Rc::new(10);
let b = Rc::clone(&a);
println!("a: {}, b: {}", a, b);
```

#### Arc<T> (Atomic Reference Counted)

- `Arc<T>` is similar to `Rc<T>`, but it’s thread-safe, making it suitable for concurrent programs.
- Adds atomic operations for thread safety, allowing multiple threads to safely share ownership of the same data.

Example:

```rust
use std::sync::Arc;
use std::thread;

let a = Arc::new(10);
let b = Arc::clone(&a);

let handle = thread::spawn(move || {
    println!("b: {}", b);
});
handle.join().unwrap();
```

#### RefCell<T>

- `RefCell<T>` enforces borrowing rules at runtime rather than at compile time.
- Enables mutability where it might not otherwise be allowed, allowing interior mutability.
- Often combined with `Rc<T>` for a single-threaded scenario where multiple parts of the code need shared, mutable access.

Example:

```rust
use std::cell::RefCell;

let x = RefCell::new(5);
*x.borrow_mut() += 1;
println!("x: {}", x.borrow());
```

#### Mutex<T> and RwLock<T>

- Used for safe shared access to data across multiple threads in a concurrent program.
- `Mutex<T>` allows exclusive access to data for writing, while `RwLock<T>` enables multiple readers or one writer at a time.
- Often combined with `Arc<T>` for shared ownership and thread-safe access.

Example (Mutex):

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let data = Arc::new(Mutex::new(0));

let handles: Vec<_> = (0..10).map(|_| {
    let data = Arc::clone(&data);
    thread::spawn(move || {
        let mut num = data.lock().unwrap();
        *num += 1;
    })
}).collect();

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", *data.lock().unwrap());
```

Example (RwLock):

```rust
use std::sync::{Arc, RwLock};
use std::thread;

let data = Arc::new(RwLock::new(0));

let handles: Vec<_> = (0..10).map(|_| {
    let data = Arc::clone(&data);
    thread::spawn(move || {
        let num = data.read().unwrap();
        println!("Read: {}", *num);
    })
}).collect();

for handle in handles {
    handle.join().unwrap();
}

let num = data.write().unwrap();
*num = 10;

println!("Write: {}", *data.read().unwrap());
```
