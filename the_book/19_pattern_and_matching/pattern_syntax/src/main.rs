fn main() {
    let x = 3;

    // Matching literals
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = Some(5);
    let y = 10;

    // Matching named variables
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // New y variable
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    let x = 1;

    // Multiple patterns
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    let x = 5;

    // Matching ranges of values with ..=
    match x {
        1..=5 => println!("one through five"), // Includes 5
        _ => println!("something else"),
    }

    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    struct Point {
        x: i32,
        y: i32,
    }

    let p = Point { x: 0, y: 7 };

    // Destructuring structs
    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);

    // Struct field shorthand
    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);

    // Matching by a specific field
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    // Destructing enums
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
    }

    enum Color {
        Rgb(i32, i32, i32),
        Hsv(i32, i32, i32),
    }

    enum NewMessage {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(Color),
    }

    let msg = NewMessage::ChangeColor(Color::Hsv(0, 160, 255));

    // Destructuring nested structs and enums
    match msg {
        NewMessage::ChangeColor(Color::Rgb(r, g, b)) => {
            println!("Change color to red {r}, green {g}, and blue {b}");
        }
        NewMessage::ChangeColor(Color::Hsv(h, s, v)) => {
            println!("Change color to hue {h}, saturation {h}, and value {v}");
        }
        _ => (),
    }

    // Destructuring structs and tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    foo(3, 4);

    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        // Match Some variants but values are not needed
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        // Ignoring multiple parts of a tuple
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}");
        }
    }

    let s = Some(String::from("Hello!"));

    // Variable starting with '_' (e.g. '_s') binds the value,
    // but '_' is not
    if let Some(_) = s {
        println!("found a string");
    }

    println!("{s:?}");

    struct Point3D {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point3D { x: 0, y: 0, z: 0 };

    // Ignore fields except for `x`
    match origin {
        Point3D { x, .. } => println!("x is {x}"),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        // Ignoring middle values in a tuple
        (first, .., last) => {
            // (.., second, ..) is ambiguous, cannot be compiled
            println!("Some numbers: {first}, {last}");
        }
    }

    let num = Some(4);

    match num {
        // Match guard
        Some(x) if x % 2 == 0 => println!("The number {x} is even"),
        Some(x) => println!("The number {x} is odd"),
        None => (),
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // Outer 'y' is used in a match guard, not being shadowed
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    let x = 4;
    let y = false;

    match x {
        // Match guard with multiple patterns
        4 | 5 | 6 if y => println!("yes"), // Work as like: (4 | 5 | 6) && y
        _ => println!("no"),
    }

    enum MessageID {
        Hello { id: i32 },
    }

    let msg = MessageID::Hello { id: 5 };

    match msg {
        // Test `id` with a pattern and hold a value into `id_variable`
        MessageID::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {id_variable}"),
        // `id` is tested, but the value isn't holded
        MessageID::Hello { id: 10..=12 } => println!("Found an id in another range"),
        // `id` isn't tested, but the value is holded
        MessageID::Hello { id } => println!("Found some other id : {id}"),
    }
}

// Ignore the first variable w/o warning
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}
