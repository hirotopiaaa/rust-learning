## Tutorial 28 - Trait Objects

## Table of Contents

<!-- vim-markdown-toc GFM -->

* [The Book](#the-book)
* [Summary](#summary)
* [Notes](#notes)
    * [Syntax](#syntax)
    * [Example](#example)
    * [Key Characteristics](#key-characteristics)
    * [Use Cases](#use-cases)
    * [Limitations](#limitations)

<!-- vim-markdown-toc -->

### The Book

Chapter 19 - Advanced Features (19.3 - Trait Objects)

### Summary

In Rust, trait objects are a way to achieve polymorphism by using traits to define behavior. They allow you to work with different types that implement the same trait without knowing their concrete types at compile time.

A `trait object` is a dynamically dispatched object that uses a trait as its interface. Unlike generic traits, which are resolved at compile time (static dispatch), trait objects defer type resolution to runtime (dynamic dispatch). They are typically created using references (`&`) or smart pointers like `Box`.

Trait objects are a powerful feature in Rust, enabling flexible and extensible designs while maintaining type safety and avoiding many pitfalls of inheritance-based object-oriented designs.

### Notes

#### Syntax

You create a trait object by specifying a reference to a trait, like so:

```rust
&dyn Trait
Box<dyn Trait>
Rc<dyn Trait>
Arc<dyn Trait>
```

> [!NOTE]
> The `dyn` keyword indicates dynamic dispatch, which means that method calls on the trait object are resolved at runtime.

#### Example

```rust
trait Draw {
    fn draw(&self);
}

struct Circle;
struct Square;

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing a Circle");
    }
}

impl Draw for Square {
    fn draw(&self) {
        println!("Drawing a Square");
    }
}

fn main() {
    let shapes: Vec<Box<dyn Draw>> = vec![
        Box::new(Circle),
        Box::new(Square),
    ];

    for shape in shapes {
        shape.draw(); // Calls the draw method for the specific type
    }
}
```

#### Key Characteristics

Dynamic Dispatch:

- Trait objects use a vtable (virtual method table) for dynamic dispatch. This adds a slight runtime cost compared to static dispatch.
- Method calls are resolved using the vtable associated with the trait object.

Sized vs. Unsized:

- Trait objects are unsized because they represent a reference to a type with unknown size at compile time.
- To use a trait object, it must be behind a pointer type (e.g., &, Box, or Rc).

No Generic Constraints:

- Unlike generics, trait objects don't require the caller to know the exact type implementing the trait, making them useful for heterogeneous collections.

Object Safety:

- Not all traits can be used as trait objects. A trait is object-safe if it satisfies the following conditions:
  - All methods must have a self receiver (self, &self, or &mut self).
  - Methods must not use generics or Self as a return type (except in certain cases like Self: Sized).

Example of object-safe trait:

```rust
trait ObjectSafe {
    fn method(&self);
}
```

Non-object-safe trait:

```rust
trait NotObjectSafe {
    fn method<T>(&self, param: T); // Generic methods are not allowed
}
```

#### Use Cases

- **Heterogeneous Collections**: Storing objects of different types in a single collection that implements the same trait.
- **Dynamic Behavior**: Allowing dynamic behavior at runtime when the exact type isn't known in advance.
- **Plugins or Extensibility**: Building systems where new implementations can be added without recompiling.

#### Limitations

- **Performance Overhead**: Due to dynamic dispatch, trait objects are slightly slower than static dispatch.
- **No Access to Concrete Type**: You can't call methods or access fields specific to the concrete type.
