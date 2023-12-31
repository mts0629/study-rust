// Module: `back_of_house`
pub struct Breakfast {
    pub toast: String,      // Accessible
    seasonal_fruit: String, // Not accessible
}

// Accessible
pub enum Appetizer {
    Soup,
    Salad,
}

impl Breakfast {
    pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
            toast: String::from(toast),
            seasonal_fruit: String::from("peaches"),
        }
    }
}

fn fix_incorrect_order() {
    cook_order();
    // Refer a parent module
    super::deliver_order();
}

fn cook_order() {}
