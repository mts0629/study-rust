fn main() {
    // Irrefutable patterns
    // `let`
    let x = 5;

    // `for`
    for i in 1..6 {
        println!("{}", i)
    }

    // Function arguments
    fn _add(x: i32) -> i32 {
        x + 1
    }

    let mut num = Some(5);
    // Refutable patterns
    // `if let`
    if let Some(x) = num { println!("num = 5"); }

    // `while let`
    while let Some(x) = num {
        if x > 6 {
            println!("quit");
            break;
        } else {
            println!("num = {}", x);
            num = Some(x + 1);
        }
    }

    // Pattern `Some(x)` is refutable, but `let` accepts only irrefutable patterns
    // let Some(x) = num; // NG
    let Some(x) = num else { return }; // OK

    // Irrefutable pattern for `let else` will be warned
    let x = 5 else { return; };  // warning: irrefutable `let...else` pattern
}
