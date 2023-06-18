use std::io;

fn main() {
    // _: prefix for unused variable

    // Compile error with type annotation:
    //
    // > error[E0282]: type annotations needed
    //
    // let guess = "42".parse().expect("Not a number!");

    let _guess: u32 = "42".parse().expect("Not a number!");

    /***************/
    // Scalar types
    /***************/
    // Integer
    let _num: i8 = 127;
    let _num: u8 = 255;
    let _num: i16 = 32_767;
    let _num: u16 = 65_535;
    let _num: i32 = 2_147_483_647;
    let _num: u32 = 4_294_967_295;
    let _num: i64 = 9_223_372_036_854_775_807;
    let _num: u64 = 18_446_744_073_709_551_615;
    let _num: i128 = 170_141_183_460_469_231_731_687_303_715_884_105_727;
    let _num: u128 = 340_282_366_920_938_463_463_374_607_431_768_211_455;
    // Architecture dependent
    let _num: isize = 1;
    let _num: usize = 1;

    // Literals
    let _num: i32 = 10_000; // Decimal
    let _num: i32 = 0x2710; // Hex
    let _num: i32 = 0o23420; // Octal
    let _num: i32 = 0b10011100010000; // Binary
    let _num: u8 = b'A'; // Byte

    // Floating point
    let _x = 2.0; // f64
    let _y: f32 = 3.0; // f32

    // Addition
    let _sum = 5 + 10;

    // Subtraction
    let _difference = 95.5 - 4.3;

    // Multiplication
    let _prod = 4 * 30;

    // Division
    let _quotient = 56.7 / 32.2;
    let _floored = 2 / 3; // Results in 0

    // Remainder
    let _remainder = 43 % 5;

    // Boolean
    let _t = true;
    let _f: bool = false;

    // Character: Unicode scalar value, single-quoted ('')
    let _c = 'z';
    let _z: char = 'ℤ';
    let _heart_eyed_cat = '😻'; // Emoji

    /*****************/
    // Compound types
    /*****************/
    // Tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {}", y);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("tup: ({}, {}, {})", five_hundred, six_point_four, one);

    // Array
    let _a = [1, 2, 3, 4, 5];
    let _months = ["January", "February", "March", "April", "May", "June", "July",
                   "August", "September", "October", "November", "December"];

    // Annotate type and the number of elements
    let _a: [i32; 5] = [1, 2, 3, 4, 5];

    // Initialize with the same value
    let a = [3; 5];
    println!("a = {:?}", a);

    let a = [1, 2, 3, 4, 5];
    let _first = a[0];
    let _second = a[1];

    // Access to an array element with the specified index
    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    // May cause a runtime error:
    //
    // > Please enter an array index.
    // > 5
    // > thread 'main' panicked at 'index out of bounds: the len is 5 but the index is 5', src/main.rs:108:19
    // > note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    //
    println!("The value of the element at index {} is: {}",
             index, element);
}
