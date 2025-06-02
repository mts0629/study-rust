fn main() {
    let v1 = vec![1, 2, 3];

    // Loop by an iterator
    let v1_iter = v1.iter();
    for val in v1_iter {
        println!("Got: {val}");
    }

    // Produce a new iterator
    let v1: Vec<i32> = vec![1, 2, 3];
    // Not work because an iterator doesn't be consumed
    // let v2 = v1.iter().map(|x| x + 1);
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);
}
