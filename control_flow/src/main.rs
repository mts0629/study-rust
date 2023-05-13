fn main() {
    let number = 3;

    // if expressions, sometime called `arms`
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // The condition must be a bool
    // Compliation error: mismatched types
    // if number {
    //     println!("number was three");
    // }

    if number != 0 {
        println!("number was something other than zero")
    }

    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        // The first body which the condition evaluated to true is executed
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    // Using if in a let statement
    let number = if condition { 5 } else { 3 };

    println!("The value of number is: {number}");

    // Compliation error: incompatible types
    // let number = if condition { 5 } else { "six" };

    // Repetition with loop
    // Infinite loop
    // loop {
    //     println!("again!");
    // }

    // Return values from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2
        }
    };

    println!("The result is {result}");

    // Loop labels to disambiguate between multiple loops
    let mut count = 0;
    // Specify loop label: `'counting_up` to an outer loop
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break; // Exit the innermost loop
            }
            if count == 2 {
                break 'counting_up; // Exit the outer loop
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");

    let mut number = 3;

    // while loop
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // Loop through a collection with while
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // Loop through a collection with for
    for element in a {
        println!("the value is: {element}");
    }

    // Loop certain number of times
    // (1..4).rev() -> [3, 2, 1]
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
