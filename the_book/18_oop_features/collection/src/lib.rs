// Integer collection and its average
pub struct AverageCollection {
    list: Vec<i32>,
    average: f64,
}

impl AverageCollection {
    /******************/
    // Public interface
    /******************/
    // Constructor
    pub fn new() -> AverageCollection {
        AverageCollection {
            list: vec![],
            average: 0.0,
        }
    }

    // Add a value
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    // Remove the last value
    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    // Get an average
    pub fn average(&self) -> f64 {
        self.average
    }

    /******************/
    // Private methods
    /******************/
    // Update average
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_average() {
        let mut avg = AverageCollection::new();

        avg.add(1);
        avg.add(2);
        // Average is calculated automatically
        assert_eq!(avg.average(), 1.5);

        avg.remove();
        assert_eq!(avg.average(), 1.0);
    }
}
