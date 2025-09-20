// Draw trait
pub trait Draw {
    fn draw(&self);
}

// Button
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

// Draw trait for Button
impl Draw for Button {
    fn draw(&self) {
        let w = self.width as usize / 10usize;
        println!("[{:^w$}]", self.label, w = w);
    }
}

// Screen
pub struct Screen {
    // Having objects implement the Draw trait
    pub components: Vec<Box<dyn Draw>>,
}

// Generics and trait bounds:
// pub struct Screen<T: Draw> { ... }
// only allows a list of items with the same type

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
