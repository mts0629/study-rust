struct Counter {
    count: u32,
}

impl Iterator for Counter {
    // Associated type, decide a type on implementation
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        Some(self.count)
    }
}

fn main() {
    let mut counter = Counter { count: 0 };

    for i in 1..=5 {
        let c = counter.next();
        match c {
            Some(i) => println!("{i}"),
            None => {}
        }
    }
}
