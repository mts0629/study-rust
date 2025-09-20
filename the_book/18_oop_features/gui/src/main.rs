use gui::{Button, Draw, Screen};

// Implement a new user struct: SelectBox
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

// Draw trait for SelectBox
impl Draw for SelectBox {
    fn draw(&self) {
        let w = self.width as usize / 10usize;
        for option in self.options.iter() {
            println!("[ ][{:^w$}]", option, w = w);
        }
    }
}

fn main() {
    // Draw a screen with a SelectBox and a Button
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
