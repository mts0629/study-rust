
// Variants for the Quarter coin
#[derive(Debug)] // So we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

// Return u8 value by matched enum value
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => { // Match arm: a pattern and some code
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // Bind the variant of Coin::Quater to state
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None, // Option type
                      // If this match arm does not exist,
                      // compilation error will occur:
                      // error[E0004]: non-exhaustive patterns: `None` not covered
        Some(i) => Some(i + 1), // i binds to the value contained in Some
    }
}

fn main() {
    let penny = Coin::Penny;
    println!("{}", value_in_cents(penny));

    let quarter = Coin::Quarter(UsState::Alabama);
    println!("{}", value_in_cents(quarter));

    let _five = Some(5);
    let _six = plus_one(_five);

    if plus_one(None) == None {
        println!("None!")
    }

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // other: cover every other possible values
                                     // it should be on the last match arm
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // _: matches any value and does not bind to that value
    }

    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (), // Do nothing by the empty tuple type
    }
}

fn add_fancy_hat() {
    println!("Get the fancy hat!")
}

fn remove_fancy_hat() {
    println!("Lose the fancy hat!")
}

fn move_player(num_spaces: u8) {
    println!("Move {}", num_spaces)
}

fn reroll() {
    println!("Roll again")
}
