use std::collections::HashMap;

fn main() {
    // Create an empty hash map
    let mut scores = HashMap::new();

    // Add elements by `insert`
    // with String keys and i32 values
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    // Access a value of "Blue" by `get`
    // it returns Option<&V>, None will be returned when no value for the specified key
    let score = scores.get(&team_name).copied().unwrap_or(0); // 0 if there's no entry for the key
    println!("{score}");

    // Iterate over each key/value pair
    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let field_name = String::from("Red");
    let field_value = 30;

    scores.insert(field_name, field_value);
    // field_name and field_value are invalid at this point
    // Compilation error: `borrow of moved value: `field_name``
    // println!("{field_name}");

    // Replace a value stored with a key: "Blue"
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);

    // Insert a new key/value pair with `entry` when if the key does not already have a value
    scores.entry(String::from("Blue")).or_insert(20);
    scores.entry(String::from("Green")).or_insert(20);

    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    // Count occurrences of words by a hash map
    // update 
    for word in text.split_whitespace() { // Return an iterator over sub-sliced splitted by whitespace
        let count = map.entry(word).or_insert(0);
        *count += 1; // Dereference by `*`
    }

    println!("{:?}", map);

    // HashMap uses a hashing function called SipHash
    // that can provide resistace to Denial of Service (Dos) attacks involving hash tables

    // A hasher is a type that implements `BuildHasher` trait,
    // and a different hasher can be used for hash function
}
