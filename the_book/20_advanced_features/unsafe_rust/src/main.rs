fn main() {
    let mut num = 5;

    // Raw pointers
    let r1 = &raw const num;
    let r2 = &raw mut num;

    // Raw pointer to a specified memory address
    let address = 0x012345usize;
    let r = address as *const i32;

    unsafe {
        // Dereference raw poiners
        println!("r1 is : {}", *r1);
        println!("r2 is : {}", *r2);
    }
}
