fn main() {
    let x = Some(1);

    // match arms
    let y = match x {
        None => None,
        Some(i) => Some(i + 1),
    };
    println!("{}", y.unwrap());

    let favorite_color: Option<&str> = None;
    let is_tuesday = true;
    let age: Result<u8, _> = "34".parse();

    // Conditional if - let expressions
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    let (tx, rx) = std::sync::mpsc::channel();
    std::thread::spawn(move || {
        for val in [1, 2, 3] {
            tx.send(val).unwrap();
        }
    });

    // while - let conditional loop
    while let Ok(value) = rx.recv() {
        println!("{value}");
    }

    let v = vec!['a', 'b', 'c'];

    // for loop
    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }

    // let statement
    let (x, y, z) = (1, 2, 3);
    println!("{x}, {y}, {z}");

    let point = (3, 5);
    print_coordinates(&point);
}

// Function parameters
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({x}, {y})");
}
