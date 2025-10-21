use std::slice;

unsafe fn dangerous() {
    // Something unsafe
}

// Split a slice at the given index
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr(); // Raw pointer

    assert!(mid <= len);

    unsafe {
        // Call unsafe codes
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}

// Use Foreign Function Interface (FFI)
unsafe extern "C" {
    // Import functions from C standard library
    fn exp(input: f64) -> f64;

    // Function marking as `safe` can be called outside of `unsafe` block
    safe fn abs(input: i32) -> i32;
}

// Static variable
static HELLO_WORLD: &str = "Hello, world!";

static mut COUNTER: u32 = 0;

/// SAFETY: Calling this from more than a single thread at a time is undefined
/// behavior, so you *must* guarantee you only call it from a single thread at
/// a time
unsafe fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// Unsafe trait
unsafe trait Foo {
    // Methods go here
}

unsafe impl Foo for i32 {
    // Method implementation go here
}

union MyUnion {
    f1: u32,
    f2: f32,
}

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

    unsafe {
        // Call unsafe function
        dangerous();
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    // let (a, b) = r.split_at_mut(3);
    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // Create a slice from an address
    // let address = 0x012345usize;
    let address = &raw const v;
    let r = address as *mut i32;
    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 3) };

    unsafe {
        println!("exp(2.17) according to C: {}", exp(2.17));
    }

    println!("Absolute value of -3 according to C: {}", abs(-3));

    println!("name is: {HELLO_WORLD}");

    unsafe {
        /// SAFETY: This is only called from a single thread in `main`
        add_to_count(3);
        println!("COUNTER: {}", *(&raw const COUNTER));
    }

    let u = MyUnion { f1: 1 };

    unsafe {
        // Access fields of union
        println!("{}", u.f1);
    }
}
