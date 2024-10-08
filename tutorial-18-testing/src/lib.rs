#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add(left: i32, right: i32) -> i32 {
    left + right
}

pub fn print_hello() {
    println!("Hello, world!");
}

/*
 * For convention, tests are placed in a tests module.
 * We write tests in the same file as the code that we are testing.
 */
#[cfg(test)]
mod tests {
    // refer to the parent module using super:: syntax
    // child modules can use the parent module's code, even if the code is private
    use super::*;

    // functions are tests if they are annotated with the #[test] attribute
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        // assert_eq!(result, 5);
    }

    #[test]
    fn another_it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn print_hello_test() {
        print_hello();
    }

    #[test]
    fn larget_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        // assert takes the second argument as the error message
        assert!(
            larger.can_hold(&smaller),
            "The larger rectangle should be able to hold the smaller rectangle"
        );

        assert!(
            smaller.can_hold(&larger),
            "The smaller rectangle should not be able to hold the larger rectangle"
        );
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        // code that takes an hour to run
    }
}
