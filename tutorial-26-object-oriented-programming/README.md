## Tutorial 26 - Object Oriented Programming

## Table of Contents

<!-- vim-markdown-toc GFM -->

* [The Book](#the-book)
* [Summary](#summary)
* [Notes](#notes)
    * [Encapsulation](#encapsulation)
    * [Polymorphism with Traits](#polymorphism-with-traits)
    * [Inheritance (Using Traits to Define Shared Behavior)](#inheritance-using-traits-to-define-shared-behavior)
    * [Ownership and Borrowing](#ownership-and-borrowing)

<!-- vim-markdown-toc -->

### The Book

Chapter 17: Object Oriented Programming

### Summary

Rust’s approach to OOP is more focused on composition and explicit behavior through traits rather than inheritance hierarchies and implicit behavior. This encourages safe, performant code while still allowing for OOP principles in a way that fits with Rust’s ownership model.

### Notes

Object-Oriented Programming (OOP) in Rust takes a slightly different approach compared to traditional object-oriented languages like Java or C++. Rust does not have "classes" in the same sense but provides mechanisms to implement OOP concepts like encapsulation, polymorphism, and inheritance (to some extent) through structs, traits, and modules. Here’s a breakdown of how these concepts are approached in Rust:

#### Encapsulation

In Rust, encapsulation is achieved using structs and modules to define and restrict access to data. By default, all fields in a struct are private and can be made accessible using `pub` if desired. You can control what is exposed outside a module and encapsulate the internal details effectively.

```rust
pub struct User {
    username: String,
    email: String,
    age: u8,
}

impl User {
    pub fn new(username: String, email: String, age: u8) -> User {
        User { username, email, age }
    }

    pub fn get_email(&self) -> &str {
        &self.email
    }
}
```

In this example, fields are private by default, but functions like `new` and `get_email` provide controlled access to the data, preserving encapsulation.

#### Polymorphism with Traits

Rust does not support polymorphism through inheritance as in traditional OOP languages. Instead, Rust uses _traits_ to define shared behavior across different types. Traits are similar to interfaces in other languages and can be implemented by any type. This allows you to achieve polymorphism in a way that’s more flexible and idiomatic to Rust.

```rust
trait Drawable {
    fn draw(&self);
}

struct Circle;
struct Square;

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle");
    }
}

impl Drawable for Square {
    fn draw(&self) {
        println!("Drawing a square");
    }
}

fn draw_shape(shape: &dyn Drawable) {
    shape.draw();
}
```

In this example, both `Circle` and `Square` implement the `Drawable` trait, allowing us to call `draw` on any object that implements `Drawable`.

#### Inheritance (Using Traits to Define Shared Behavior)

Rust does not support inheritance in the classical sense (i.e., inheriting fields and methods from a parent class). Instead, it allows for _composition over inheritance_. You can define traits and have multiple structs implement those traits to share common behavior.

If you need something similar to inheritance, Rust allows you to define traits that depend on other traits, creating trait hierarchies to represent more complex behavior.

```rust
trait Shape {
    fn area(&self) -> f64;
}

trait Drawable: Shape {
    fn draw(&self);
}
```

Here, `Drawable` depends on `Shape`, so any type implementing `Drawable` must also implement `Shape`. This lets you create behavior hierarchies without true inheritance.

#### Ownership and Borrowing

Rust’s ownership model influences its OOP design significantly. In Rust, data is managed through ownership, borrowing, and lifetimes, which prevent common issues like null pointers and data races. This means that Rust’s approach to OOP has more safety checks and is generally more memory efficient.
