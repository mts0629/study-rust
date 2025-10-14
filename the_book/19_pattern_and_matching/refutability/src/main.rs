fn main() {
    // Pattern `Some(x)` is refutable, but `let` accepts only irrefutable patterns
    // let Some(x) = Some(1);
    let Some(x) = Some(1) else { return }; // OK

    // Irrefutable pattern for `if let`
    let x = 5 else { return };
}
