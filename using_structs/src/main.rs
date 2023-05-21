// Struct definition
struct User {
    active: bool,
    username: String,   // Using &str requires to specify lifetimes
    email: String,
    sign_in_count: u64,
}

// Tuple struct
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-like struct
struct AlwaysEqual;

fn main() {
    // Declare an instance of the `User` struct
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // Access to a specific field
    user1.email = String::from("anotheremail@example.com");
    println!("{0}", user1.email);

    let user1 = build_user(
        String::from("someusername123"),
        String::from("someone@example.com")
    );

    // Update syntax: move unspecified values from another instance
    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("{0}", user2.email);

    let green = Color(0, 255, 0);
    let origin = Point(0, 0, 0);

    println!("{}", green.1);

    // Define an instance of unit-like struct
    let subject = AlwaysEqual;
}

// Function which returns a new instance of the struct
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

