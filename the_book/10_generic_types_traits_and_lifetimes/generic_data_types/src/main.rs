// Using a generic type parameter T
// std::cmp::PartialOrd is a trait, for type which an operator '>' can be used to
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Struct with a generic type
struct Point<T> {
    x: T,
    y: T,
}

// Use multiple generic types
struct Point2<T, U> {
    x: T,
    y: U,
}

// Option<T> and Result<T, E> have multiple generic types

// Method with a generic type
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Method only applied for Point<f32>
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point3<X1, Y1> {
    x: X1,
    y: Y1,
}

// Method using generic types different from its definition
impl<X1, Y1> Point3<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Point3<X2, Y2>) -> Point3<X1, Y2> {
        Point3 {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let _integer = Point { x: 5, y: 10 };
    let _float = Point { x: 1.0, y: 4.0 };
    // let wont_work = Point { x: 5, y: 4.0 }; // x and y must be the same type

    let _both_integer = Point2 { x: 5, y: 10 };
    let _both_float = Point2 { x: 1.0, y: 4.0 };
    let _integer_and_float = Point2 { x: 5, y: 4.0 };

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    let p = Point { x: 1.0, y: 2.0 };
    println!("distance = {}", p.distance_from_origin());


    let p1 = Point3 { x: 5 , y: 10.4 };
    let p2 = Point3 { x: "Hello" , y: 'c' };

    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    // Rust peforms monomorphization
    let _integer = Some(5);
    let _float = Some(5.0);

    /* The above is equivalent to:

    enum Option_i32 {
        Some(i32),
        None,
    }

    enum Option_f64 {
        Some(f64),
        None,
    }
    
    let _integer = Option_i32:Some(5);
    let _float = Option_f64:Some(5.0);
    */
}
