struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Create an implementation block for the Rectangle struct
// This is similar to a class in OOP
// Methods are defined within the implementation block
// Methods are defined with the self keyword
impl Rectangle {
    // Method to calculate the area of the rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method to check if the rectangle can hold another rectangle
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    println!("Hello, world!");
    // Creating an instance of User
    let mut user1: User = User {
        email: String::from("hello@example.com"),
        username: String::from("hello"),
        active: true,
        sign_in_count: 1,
    };

    let name = user1.username;
    user1.username = String::from("hello2");

    let user2 = build_user(String::from("kyle@mail.com"), String::from("kyle"));
    let user3 = User {
        email: String::from("james@mail.com"),
        username: String::from("james"),
        // get remaining field values from user2
        ..user2
    };

    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    // Print the rectangle with debug formatting
    println!("rect: {:#?}", rect);

    // Call the area function with a reference to the rectangle
    println!("The area of the rectangle is {}", area(&rect));

    // With the implementation block, we can call the area method on the Rectangle struct
    println!("The area of the rectangle is {}", rect.area());

    let rect1 = Rectangle {
        width: 20,
        height: 40,
    };

    let rect2 = Rectangle {
        width: 50,
        height: 60,
    };

    println!("Can rect1 hold rect1? {}", rect.can_hold(&rect1));
    println!("Can rect1 hold rect2? {}", rect.can_hold(&rect2));

    let rect3 = Rectangle::square(30);
    println!("rect3: {:#?}", rect3);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

// Use & to borrow the struct instead of taking ownership
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
