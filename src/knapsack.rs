//! A Knapsack that contains several items who has a total weight and a total value

//! Provides an object that has a limit of weight and where algorithms we will develop tend to increase its value as much as they can
//! Knapsacks are represented with four values
//! - A maximum weight (max_weight). The total weight of the items in the Knapsack can never go beyond that limit. Otherwise the Knapsack is not valid and must be instantly deleted.
//! - A total weight that represents the sum of the weights of all the items that a Knapsack contains.
//! - A total value that represents the sum of the values of all the items in a Knapsack. The goal of the algorithms provided in this project is to maximize this total_weight.
//! - A configuration that shows precisely which item is contained in the Knapsack.

use std::collections::{HashSet, HashMap};

mod item;
pub use self::item::Item;

pub struct Knapsack {
    max_weight: u32,
    total_weight: u32,
    total_value: u32,
    configuration: HashSet<String>,
} 

impl Knapsack {
    pub fn new(max_weight: u32) -> Self {
        Knapsack {
            max_weight,
            total_weight: 0,
            total_value: 0,
            configuration: HashSet::new(),
        }
    }

    pub fn show(&self, all_items: &HashMap<String, Item>) {
        println!("Knapsack [Weight : {} / {} ; Value : {}]", self.total_weight, self.max_weight, self.total_value);
        for x in self.configuration.iter() {
            print!("\t");
            if let Some(val) = all_items.get(x) {
                val.show();
            }
        }
    }

    pub fn add_item(&mut self, item: &Item) -> bool {
        if self.total_weight + item.weight() > self.max_weight {
            return false;
        }
        if self.configuration.insert(item.name().clone()) {
            self.total_weight += item.weight();
            self.total_value += item.value();
            return true;
        }
        false
    }

    pub fn withdraw_item(&mut self, item: &Item) -> bool {
        if self.configuration.remove(item.name()) {
            self.total_weight -= item.weight();
            self.total_value -= item.value();
            return true;
        }
        false
    }
}