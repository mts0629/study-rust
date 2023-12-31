// Define a module: `front_of_house`
mod front_of_house {
    // Submodule: `hosting`
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    // Submodule: `serving`
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,      // Accessible
        seasonal_fruit: String, // Not accessible
    }

    // Accessible
    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        // Refer a parent module
        super::deliver_order();
    }

    fn cook_order() {}
}

// re-exporting:
// An external module can use add_to_waitlist() with
// restaurant::hosting::add_to_waitlist()
pub use crate::front_of_house::hosting;

// Specifying a full path to a function makes the function unclear
// whether it is the local one or not
// use crate::front_of_house::hosting::add_to_waitlist;

// On the other hand, struct is specified with the full path
use std::collections::HashMap;

use rand::Rng;

// Function which refers a crate's function
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // Won't be compile because seasonal_fruit is private
    // meal.seasonal_fruit = String::from("blueberries");

    // Access to a public enum
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    // Shortcut a module path by use crate::front_of_house::hosting;
    hosting::add_to_waitlist();

    let mut map = HashMap::new();
    map.insert(1, 2);

    let secret_number = rand::thread_rng().gen_range(1..=100);
}

mod customer {
    pub fn eat_at_restaurant() {
        // Won't be compile because use keyword does not work in this module
        // hosting::add_to_waitlist();

        // super keyword works
        super::hosting::add_to_waitlist();
    }
}

// Two Result types are specified by using their parent modules
/*
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
}

fn function2() -> io::Result<()> {
    // --snip--
}
*/

// Rename std::io::Result by `IoResult`
/*
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
}

fn function2() -> IoResult<()> {
}
*/

// Using nested pathes to bring multipl items with the same prefix
// use std::cmp::Ordering, std::io
use std::{cmp::Ordering, io};

// use std::io, std::io::Write
// use std::io::{self, Write};

// Bring all public items defined in a path into scope by glob operator: `*`
use std::collections::*;
