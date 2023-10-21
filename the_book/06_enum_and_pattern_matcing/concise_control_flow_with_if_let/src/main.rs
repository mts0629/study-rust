fn main() {
    let config_max = Some(3u8);
    // Only want to execute code if the value is Some variant
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // if let: bind the value inside the Some and work as match arm
    // It may lose exhaustive checking
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    #[derive(Debug)]
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

    {
        let coin = Coin::Quarter(UsState::Alabama);

        let mut count = 0;
        // Only one match case and others
        match coin {
            Coin::Quarter(state) => println!("State quarter from {:?}!", state),
            _ => count += 1,
        }
    }

    {
        let coin = Coin::Quarter(UsState::Alabama);

        let mut count = 0;
        // The same behavior with the above match arm, by if let
        if let Coin::Quarter(state) = coin {
            println!("State quarter from {:?}!", state);
        } else {
            count += 1;
        }
    }
}
