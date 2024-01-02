fn main() {
    // Create a vector of i32
    let _v: Vec<i32> = Vec::new();

    // Create a vector with initial values
    // which type is assumed as i32
    let mut v = vec![1, 2, 3];

    // Add elements
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    // Iterating over the elements in a vector
    for i in &v {
        println!("{}", i);
    }

    // Iterating over mutable references to elements
    for i in &mut v {
        *i += 50;
    }

    // Read elements via indexing
    let third: &i32 = &v[2];
    println!("The third element is {third}");

    // Read elements by using get method
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Runtime error: `index out of bounds`
    // let does_not_exist = &v[100];

    // None is returned
    let does_not_exist = v.get(100);

    /*{
        let mut v = vec![1, 2, 3, 4, 5];

        let first = &v[0];  // Immutable borrow

        // Compilation error:
        // `cannot borrow `v` as mutable because it is also borrowed as immutable`
        v.push(6);

        println!("The first element is: {first}");  // Immutable borrow
    }*/

    // Column of a spreadsheet
    enum SperadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // A vector of the enum
    let row = vec![
        SperadsheetCell::Int(3),
        SperadsheetCell::Text(String::from("blue")),
        SperadsheetCell::Float(10.12),
    ];

    {
        let a = vec![1, 2, 3, 4];
    } // a goes out of scope and is freed here

    // Compilation error: `cannot find value `a` in this scope
    // for e in &a {
    //     println!("{}", e);
    // }
}
