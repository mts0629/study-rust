struct CustomSmartPointer {
    data: String,
}

// Drop trait
impl Drop for CustomSmartPointer {
    // drop, called when an object goes out of scope
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointers created.");
    drop(c); // Explicit drop of c
    println!("CustomSmartPointer dropped before the end of main.");
} // _d.drop() called automatically
