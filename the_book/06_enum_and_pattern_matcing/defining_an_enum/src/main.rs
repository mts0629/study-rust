// enum for a kind of the IP address
enum IpAddrKind {
    V4,
    V6,
}

fn main() {
    // Create instances of two variants of IpAddrKind
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    {
        #[allow(dead_code)]
        // Using enum for a field of the struct
        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }

        let _home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };

        let _loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
    }

    {
        #[allow(dead_code)]
        // Attach data to each variant of the enum directly
        enum IpAddr {
            V4(String),
            V6(String),
        }

        // IpAddr::V4() is a function call
        // that takes String argument and returns an instance of IPAddr type
        let _home = IpAddr::V4(String::from("127.0.0.1"));

        let _loopback = IpAddr::V4(String::from("::1"));
    }

    {
        #[allow(dead_code)]
        // Attach data which have different types and amounts
        // for each variant of the enum
        // Also different struct can be set
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }

        let _home = IpAddr::V4(127, 0, 0, 1);

        let _loopback = IpAddr::V6(String::from("::1"));
    }

    #[allow(dead_code)]

    // enum which has four variants with different data types
    #[derive(Debug)]
    enum Message {
        Quit, // No associated data
        Move { x: i32, y: i32 },    // Two named i32 fields
        Write(String), // Single String
        ChangeColor(i32, i32, i32), // Three i32 values
    }

    // Similer to the above enum, but they are diffrent type each other
    // struct QuitMessage;
    // struct MoveMessage {
    //     x: i32,
    //     y: i32,
    // }
    // struct WriteMessage(String);
    // struct ChangeColorMessage(i32, i32, i32);

    // Define methods to enum
    impl Message {
        // Do debug print of enum
        fn call(&self) {
            println!("{:?}", self)
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();

    // Option is defined enum in stdlib, for the value which can be nothing
    // enum Option<T> {
    //     None,
    //     Some(T),
    // }

    // The types are infered from specified values
    let _some_number = Some(5);
    let _some_char = Some('e');

    let _absent_number: Option<i32> = None;

    // T and Option<T> are diffent types
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // So addition of i8 and Option<i8> is not allowed
    // error[E0277]: cannot add `Option<i8>` to `i8`
    // let sum = x + y;

    // Valid value can be get by unwrap()
    println!("{}", x + y.unwrap())
}
