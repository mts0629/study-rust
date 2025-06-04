fn multiply_accumurate(v1: Vec<i32>, v2: Vec<i32>) -> i32 {
    let mut s = 0;

    for i in 0..v1.len() {
        s += v1[i] * v2[i]
    }

    s
}

fn main() {
    let mut v1: Vec<i32> = Vec::new();
    for _ in 0..1000 {
        v1.push(1);
    }

    let mut v2: Vec<i32> = Vec::new();
    for _ in 0..1000 {
        v2.push(2);
    }
   v2.push(2);

    println!("{}", multiply_accumurate(v1, v2));
}
