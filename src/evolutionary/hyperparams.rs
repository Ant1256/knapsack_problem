//! Hyperparameters for a genetic algorithm
//!
//! Provides a struct that holds four parameters that greatly influences the progress of the genetic algorithm.
//!
//! The four parameters are :
//! max_children : the maximum amount of children that can indivual can do, the more an individual has value in his knapsack the more he will do children
//! It is set to 20 by default. It cannot be 0.
//! growth_factor : the growth factor of the exponential curve that defines how many children an indicidual can procucen, if it is low the curve will be flat, if it is high the curve will be more vetical.
//! It is set to 1 by default. It cannot take null or negative values.
//! max_total_value : the maximum theoretical value that could take the knapsack.
//! By default, it is the sum of the value of all objects given. It cannot be 0.
//! mut_chance : the reciprocal of the chance for an individual to mutate during his creation. In other words an individual has 1 / mut_chance to mutate. Thus, the more it is ellevated, the less mutations are likely to appear.
//! mut_chance has a default value of 10 and cannot be 0.

use super::super::knapsack::Item;

pub struct Hyperparameters {
    pub max_children: u32,
    pub growth_factor: f64,
    pub max_total_value: u32,
    pub mut_chance: usize,
}

impl Hyperparameters {
    pub fn new(all_items: &Vec<Item>) -> Self {
        Self {
            max_children: 20,
            growth_factor: 1.0,
            max_total_value: sum_of_values(all_items),
            mut_chance: 10,
        }
    }

    // Setters

    pub fn change_max_children(&mut self, new_val: u32) {
        if new_val == 0 {
            panic!("The maximum amount of children cannot be zero.");
        }
        self.max_children = new_val;
    }

    pub fn change_growth_factor(&mut self, new_val: f64) {
        if new_val <= 0.0 {
            panic!("The groth factor must be strictly positive.");
        }
        self.growth_factor = new_val;
    }

    pub fn change_max_total_value(&mut self, new_val: u32) {
        if new_val == 0 {
            panic!("The maximum total value cannot be zero.");
        }
        self.max_total_value = new_val;
    }

    pub fn change_mut_chance(&mut self, new_val: usize) {
        if new_val == 0 {
            panic!("The mut chance cannot be zero.");
        }
        self.mut_chance = new_val;
    }
}

fn sum_of_values(all_items: &Vec<Item>) -> u32 {
    let mut res: u32 = 0;
    for item in all_items {
        res += item.value();
    }
    res
}