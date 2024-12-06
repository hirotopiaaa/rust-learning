## Tutorial 14 - Enums and Pattern Matching

### The Book

Chapter 6: Enums and Pattern Matching

### Summary

- Enums are a way to define a type by enumerating its possible variants.
- Enums can have associated data.
- Enums can have methods.
- Enums can be used in pattern matching.

### Notes

Option is an enum that is defined in the standard library. Option is used to represent a value that may or may not be present. Option is used to represent a value that may or may not be present. Option is used to represent a value that may or may not be present.

e.g.

```rust
enum Option<T> {
    Some(T),
    None,
}
```

match is a keyword in Rust that is used to perform pattern matching. match is a keyword in Rust that is used to perform pattern matching. match is a keyword in Rust that is used to perform pattern matching. match is a keyword in Rust that is used to perform pattern matching.

e.g.

```rust
fn main() {
    let some_u8_value = Some(0u8);
    match some_u8_value {
        // if 3 is the value, print "three"
        Some(3) => println!("three"),
    }
}
```

### How to reference a workspace from another workspace in rust

In Rust, you can define an `enum` with associated types for its variants. Each variant of the `enum` can hold different types of data. Here's how to define and use such enums:

### Example: Enum with Types

```rust
enum Shape {
    Circle(f64),           // Holds a single `f64` representing the radius
    Rectangle {            // Holds named fields for width and height
        width: f64,
        height: f64,
    },
    Triangle(f64, f64, f64), // Holds three `f64`s for the sides of the triangle
}
```

### Working with Enum Variants

You can create instances of these variants and match against them to access their associated data:

```rust
fn main() {
    let circle = Shape::Circle(10.0);
    let rectangle = Shape::Rectangle { width: 5.0, height: 10.0 };
    let triangle = Shape::Triangle(3.0, 4.0, 5.0);

    print_shape_info(circle);
    print_shape_info(rectangle);
    print_shape_info(triangle);
}

fn print_shape_info(shape: Shape) {
    match shape {
        Shape::Circle(radius) => {
            println!("Circle with radius: {}", radius);
        }
        Shape::Rectangle { width, height } => {
            println!("Rectangle with width: {} and height: {}", width, height);
        }
        Shape::Triangle(a, b, c) => {
            println!("Triangle with sides: {}, {}, {}", a, b, c);
        }
    }
}
```

### Key Points

1. **Tuple Variants**:

   - Variants can have unnamed fields, like `Circle(f64)` or `Triangle(f64, f64, f64)`.

2. **Struct Variants**:

   - Variants can use named fields, like `Rectangle { width: f64, height: f64 }`.

3. **Unit Variants**:

   - Variants can also have no data, acting as simple tags, e.g., `enum Status { Success, Failure }`.

4. **Usage with Functions**:
   - Enums with types are particularly useful for creating type-safe APIs or modeling data that can take on multiple forms.

Would you like to see more complex patterns, such as enums with lifetimes or generics?

### Can an Enum Variant Exist Without an Associated Type?

Yes, an enum variant in Rust can exist without an associated type. These are called **unit variants**, and they work like constant values. They don't hold any data and are often used as simple tags or states.

### Example: Enum with Unit Variants

```rust
enum Status {
    Success,
    Failure,
    Pending,
}
```

### Using Unit Variants

You can use such an enum to represent discrete states or choices. Here's an example:

```rust
fn main() {
    let current_status = Status::Pending;

    match current_status {
        Status::Success => println!("The operation was successful!"),
        Status::Failure => println!("The operation failed."),
        Status::Pending => println!("The operation is still pending."),
    }
}
```

### Why Use Unit Variants?

1. **Tags or Flags**:
   Unit variants are ideal for scenarios where you only need to differentiate between distinct states or tags without extra data.

2. **Readable Code**:
   Using enum variants instead of integers or strings makes the code more expressive and type-safe.

### Mixed Enum Variants

You can mix unit variants with variants that hold data:

```rust
enum Message {
    Quit,                    // Unit variant
    Write(String),           // Tuple variant
    ChangeColor { r: u8, g: u8, b: u8 }, // Struct variant
}

fn main() {
    let msg = Message::Quit;

    match msg {
        Message::Quit => println!("Quit message received."),
        Message::Write(text) => println!("Message: {}", text),
        Message::ChangeColor { r, g, b } => {
            println!("Change color to RGB({}, {}, {})", r, g, b);
        }
    }
}
```

This flexibility allows `enum` to represent diverse data structures in Rust effectively. Would you like further examples or practical use cases?
