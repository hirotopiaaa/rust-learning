use rand::{CryptoRng, Rng};
use std::io::{self, Write};

// By default, all items (functions, methods, structs, enums, modules, and constants) are private
// to the parent module.
// if a module is defined in a file, the file should be named the same as the module.
// import the module from front_of_house.rs
// put the module contents in a file with the same name as the module
// alternatively,
// import the module from front_of_house/
// if a module is defined in a directory, the directory should contain a file named `mod.rs`.
mod front_of_house;

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // start from the current module
    front_of_house::hosting::add_to_waitlist();
}

// Example #2
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        // access the sibling function within the same module
        cook_order();
        // super keyword is used to access the parent module
        super::serve_order();
    }

    fn cook_order() {}
}

// Example #3

mod back_of_house2 {
    pub struct Breakfast {
        // By default, struct fields are private
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant2() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house2::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if uncommented because seasonal_fruit is private
    // meal.seasonal_fruit = String::from("blueberries");
}

// Example #4
// use keyword

// self refers to the current module
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant3() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
