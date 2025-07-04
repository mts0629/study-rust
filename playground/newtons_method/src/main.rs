fn newtons_method(f: fn(f64) -> f64, df: fn(f64) -> f64, x0: f64, delta: f64) -> f64 {
    let mut x = x0;
    let mut prev_x = x0 + delta;

    while (x - prev_x).abs() > delta {
        prev_x = x;
        x = x - (f(x) / df(x));
    }

    x
}

fn main() {
    let f = |x: f64| x * x - 2.0; // f(x)=x^2-2
    let df = |x: f64| 2.0 * x; // df(x)=2x

    let x0 = 3.0; // Initial value
    println!("x^2-2=0, x={}", newtons_method(f, df, x0, 1e-6));
}
