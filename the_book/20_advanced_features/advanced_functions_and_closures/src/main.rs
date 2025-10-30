fn add_one(x: i32) -> i32 {
    x + 1
}

// Call a function twice passed by function pointer, with a specified argument
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {answer}");

    // Convert a vector of integers to a vector of strings with map and closure
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(|i| i.to_string()).collect();

    // Same conversion with map and function pointer (fully qualified syntax is required)
    let list_of_numbers = vec![1, 2, 3];
    let list_of_strings: Vec<String> = list_of_numbers.iter().map(ToString::to_string).collect();

    enum Status {
        Value(u32),
        Stop,
    }

    // Use an enum initializer functions in map
    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}
