use core::time::Duration;
use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue
}

struct Inventory {
    shirts: Vec<ShirtColor>
}

impl Inventory {
    // Get a user preference if it exists,
    // otherwise get the most stocked color
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        // Closure with no argument,
        // call self.most_stocked() of the current Inventory instance
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    // Return the most stocked shirt color
    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }
        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue]
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    // Closure with type annotations
    let _expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };

    // Closure definitions
    let _add_one_v1 = |x: u32| -> u32 { x + 1 };
    // Doesn't type inference work?
    let _add_one_v2 = |x: u32| { x + 1 }; 
    let _add_one_v3 = |x: u32| x + 1;

    let example_closure = |x| x;
    let _s = example_closure(String::from("hello"));
    // Error because a parameter of example_closure is infered as String
    // by the above function call
    // let _n = example_closure(5);

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    // Borrow is list occurs here
    let mut borrows_mutably = || list.push(7);
    // println! needs an immutable borrow, so it can't be called here
    // println!("Before calling closure: {list:?}");
    borrows_mutably();
    println!("After calling closure: {list:?}");

    let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");
    // Thread may outlive the current function,
    // so take an ownership of list by move
    thread::spawn(move || println!("From thread: {list:?}"))
        .join()
        .unwrap();

    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];
    // sort_by_key uses FnMut trait, list is mutated
    list.sort_by_key(|r| r.width);
    println!("{list:#?}");

    // Count closure calls but it can't compile
    // because sort_operations.push() moves value
    // let value = String::from("closure called");
    // let mut sort_operations = vec![];
    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });
    // println!("{list:#?}");

    // More straightforward way for count closure calls
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");
}
