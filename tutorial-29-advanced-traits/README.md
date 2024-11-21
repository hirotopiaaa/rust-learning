## Tutorial 29 - Advanced Traits

## Table of Contents

<!-- vim-markdown-toc GFM -->

* [The Book](#the-book)
* [Summary](#summary)
* [Notes](#notes)
    * [Default Implementations](#default-implementations)
    * [Trait Bounds](#trait-bounds)
    * [Associated Types](#associated-types)
    * [Calling Methods with the Same Name](#calling-methods-with-the-same-name)
        * [Inherent and Trait Methods](#inherent-and-trait-methods)
        * [Multiple Trait Implementations](#multiple-trait-implementations)
        * [Fully Qualified Syntax](#fully-qualified-syntax)
    * [Supertraits](#supertraits)
    * [Newtype Pattern](#newtype-pattern)
    * [Dynamic Dispatch and dyn](#dynamic-dispatch-and-dyn)
    * [Returning Traits with impl Trait](#returning-traits-with-impl-trait)

<!-- vim-markdown-toc -->

### The Book

Chapter 19 - Advanced Features (Traits)

### Summary

Advanced traits in Rust let developers:

- Define sophisticated relationships between types.
- Reduce boilerplate using associated types and default implementations.
- Achieve polymorphism with traits in both compile-time (static dispatch) and runtime (dynamic dispatch) contexts.

### Notes

#### Default Implementations

Default implementations are a way to provide a default implementation for a trait method. This allows the implementor to use the default implementation if they don't want to provide their own implementation.

Traits can provide default implementations for some methods. This allows implementors to only override the methods they need.

```rust
trait Greeting {
    fn hello(&self) {
        println!("Hello, world!");
    }
}

struct Person;

impl Greeting for Person {} // Inherits the default hello implementation.
```

- Implementors can still override default methods if custom behavior is required.

#### Trait Bounds

Trait bounds are a way to specify that a generic type parameter must implement a certain trait. This allows the implementor to use the methods from the trait within the generic type.

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

- Benefit: Enforces constraints on generic types.

#### Associated Types

Associated types are a way to define a placeholder type within a trait definition for the implementor to define. This allows the trait to use types that are unknown until the trait is implemented.

Traits can define associated types, which act as placeholders within the trait definition. This reduces boilerplate code by letting implementors specify the concrete type.

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter;

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        // Implementation here
        Some(42)
    }
}
```

- Benefit: Simplifies code when a trait has a type that is used multiple times within the trait.
- Without associated types, you’d need to use generics everywhere.

#### Calling Methods with the Same Name

When a trait has a method with the same name as a method in the implementor, the implementor's method will take precedence.

In Rust, you may encounter scenarios where multiple methods with the same name are accessible due to overlapping traits or inherent implementations. Rust provides a way to resolve such ambiguities explicitly.

##### Inherent and Trait Methods

If a type has a method with the same name as a trait method, Rust allows you to specify which method to call.

Example: A struct with a method fly, and a trait Fly also provides a fly method.

```rust
struct Human;

trait Fly {
    fn fly(&self);
}

impl Fly for Human {
    fn fly(&self) {
        println!("Human is flying with a jetpack!");
    }
}

impl Human {
    fn fly(&self) {
        println!("Human is walking on the ground.");
    }
}

fn main() {
    let person = Human;

    // Call the inherent method
    person.fly();

    // Call the trait method explicitly
    Fly::fly(&person);
}

// Output:
// Human is walking on the ground.
// Human is flying with a jetpack!
```

##### Multiple Trait Implementations

If a type implements multiple traits that define a method with the same name, you can disambiguate the call by specifying the trait name.

Example: Two traits, Pilot and Wizard, both provide a method named fly.

```rust
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("Pilot flying the plane!");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Wizard flying on a broomstick!");
    }
}

impl Human {
    fn fly(&self) {
        println!("Human walking.");
    }
}

fn main() {
    let person = Human;

    // Call the inherent method
    person.fly();

    // Call the `Pilot` trait method
    Pilot::fly(&person);

    // Call the `Wizard` trait method
    Wizard::fly(&person);
}

// Output:
// Human walking.
// Pilot flying the plane!
// Wizard flying on a broomstick!
```

##### Fully Qualified Syntax

The fully qualified syntax allows you to call a method from a specific trait.

- Inherent methods always take precedence over trait methods if no explicit disambiguation is used.
- Use <Type as Trait>::method(&instance) to call a trait method explicitly.
- Explicit trait calls are especially important when dealing with multiple traits defining methods with the same name.
- These features enable Rust to maintain clarity and control in codebases with overlapping trait and method definitions.

```rust
trait Animal {
    fn sound(&self);
}

struct Dog;

impl Animal for Dog {
    fn sound(&self) {
        println!("Woof!");
    }
}

impl Dog {
    fn sound(&self) {
        println!("Bark!");
    }
}

fn main() {
    let dog = Dog;

    // Call the inherent method
    dog.sound();

    // Call the trait method explicitly
    <Dog as Animal>::sound(&dog);
}

// Output:
// Bark!
// Woof!
```

#### Supertraits

Supertraits are a way to define a trait that requires another trait to be implemented. This allows the implementor to use the methods from the supertrait.

A trait can require that another trait is also implemented. These are known as supertraits.

```rust
trait Write {
    fn write(&self);
}

trait Summary: Write {
    fn summarize(&self) {
        println!("Summarizing!");
    }
}
```

- A type that implements Summary must also implement Write.

#### Newtype Pattern

The newtype pattern is a way to create a new type that wraps an existing type. This allows the implementor to provide custom behavior for the new type.

When implementing external traits on external types is not possible, the newtype pattern can be used to create a wrapper type.

```rust
struct Wrapper(Vec<String>);

impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}
```

Benefit: Work around orphan rules and extend functionality.

#### Dynamic Dispatch and dyn

Dynamic dispatch is a way to call a method on a trait object. This allows the implementor to call the method without knowing the concrete type of the object.

Rust allows for dynamic dispatch through trait objects (dyn Trait), enabling polymorphism at runtime.

```rust
fn print_summary(item: &dyn Summary) {
    item.summarize();
}
```

Trait objects use a vtable for dispatch, which incurs a slight runtime cost compared to static dispatch.

#### Returning Traits with impl Trait

Returning traits with impl Trait is a way to return a trait object from a function. This allows the implementor to return a trait object without knowing the concrete type of the object.

Traits can be used in return types for more flexibility without exposing the concrete type.

```rust
fn create_iterator() -> impl Iterator<Item = i32> {
    (1..5)
}
```
