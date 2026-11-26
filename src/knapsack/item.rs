//! Basic items that could fill a knapsack
//! 
//! Provides the most basic element of the knapsack problem : the item that fills the knapsack
//! Items are represented by three values :
//! - An id which is a unique key that identifies an item
//! - A weight which represents the total fraction of knapsack ressources that this item consumes and that, thus, will not be available for other items
//! - A value which is the gain that the knapsack problem wants to maximize

use super::super::IdGenerator;

#[derive(Debug)]
pub struct Item {
    id: usize,
    weight: u32,
    value: u32,
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Item {}

impl Clone for Item {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            weight: self.weight,
            value: self.value,
        }
    }
}

impl Item {
    pub fn new(the_id_gen: &mut IdGenerator, weight: u32, value: u32) -> Self {
        if weight == 0 || value == 0 {
            panic!("Weight and value must be positive.")
        }
        Self {
            id: the_id_gen.next(),
            weight,
            value,
        }
    }

    pub fn show(&self) {
        println!("{} [Weight : {} ; Value : {}]", self.id, self.weight, self.value);
    }

    // Getters
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn weight(&self) -> u32 {
        self.weight
    }

    pub fn value(&self) -> u32 {
        self.value
    }
}
