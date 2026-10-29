//! An approximation of the solution of the knapsack problem
//!
//! Provides a greedy solution of complexity O(n log n) where n is the number of items that can fill the Knapsack
//! This solution is very approximative. It it not and can not be considered as an exact solution.
//! 
//! The function greedy_approximation takes two parameters :
//! A vector composed of all items that the knapsack could possibly hold
//! A maximum weight for the knapsack
//! It returns a knapsack composed with a non-optimal combination that tends to maximize the total value contained by it.

use super::knapsack::{Knapsack, Item};

fn value_per_unit(item: &Item) -> f64 {
    item.value() as f64 / item.weight() as f64
}

pub fn greedy_approximation(all_items: &Vec<Item>, max_weight: u32) -> Knapsack {
    let mut all_items = all_items.clone();
    all_items.sort_by(|x, y| value_per_unit(&y).total_cmp(&value_per_unit(&x)));
    let mut res = Knapsack::new(max_weight);
    for item in all_items {
        res.add_item(&item);
    }
    res
}
