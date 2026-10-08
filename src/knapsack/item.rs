//! Basic items that could fill a knapsack
//! 
//! Provides the most basic element of the knapsack problem : the item that fills the knapsack
//! Items are represented by three values :
//! - A name which is a visual representation of an item for humans
//! - A weight which represents the total amount of knapsack ressources that this item consumes and that, thus, will not be available for other items
//! - A value which is the gain that the knapsack problem wants to maximize

pub struct Item {
    name: String,
    weight: u32,
    value: u32,
}

impl Item {
    pub fn new(name: &str, weight: u32, value: u32) -> Self {
        if weight == 0 || value == 0 {
            panic!("Weight and value must be positive.")
        }
        Self {
            name: name.to_string(),
            weight,
            value,
        }
    }

    pub fn value_per_unit(&self) -> f64 {
        self.value as f64 / self.weight as f64
    }

    pub fn is_more_valuable(&self, other: &Self) -> bool {
        self.value_per_unit() >= other.value_per_unit()
    }

    pub fn show(&self) {
        println!("{} [Weight : {} ; Value : {}]", self.name, self.weight, self.value);
    }
}
